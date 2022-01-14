//! Macros for including transformed files at compile time.
//!
//! The macros include a file similar to [`include_bytes!`], but transform the
//! data before inclusion (e.g. runs an assembler).
//!
//! The original purpose of this project was including assembled application
//! processor initialization code in [libhermit-rs] using [`include_nasm_bin!`].
//!
//! ```
//! [dependencies]
//! include-transformed = "0.1"
//! ```
//!
//! # Requirements
//!
//! This project depends on the unstable features `proc_macro_span` [#54725] and
//! `proc_macro_expand` [#90765], which are only available on the nightly
//! toolchain channel.
//!
//! [libhermit-rs]: https://github.com/hermitcore/libhermit-rs
//! [#54725]: https://github.com/rust-lang/rust/issues/54725
//! [#90765]: https://github.com/rust-lang/rust/issues/90765

#![feature(proc_macro_expand)]
#![feature(proc_macro_span)]

use std::{io::Read, path::Path, process::Command};

use proc_macro::Literal;
use quote::quote;
use syn::{parse_macro_input, LitStr};
use tempfile::NamedTempFile;

/// Simluates inclusion.
///
/// This makes sure, the proc-macro is rerun if the included file has changed.
fn simulate_inclusion(file: &LitStr) -> Result<(), proc_macro::ExpandError> {
    let src = quote! {
        include_bytes!(#file)
    };
    proc_macro::TokenStream::from(src).expand_expr().map(drop)
}

macro_rules! expand_macro_input {
    ($tokenstream:ident) => {
        match $tokenstream.expand_expr() {
            Ok(tokenstream) => tokenstream,
            Err(err) => {
                let tokenstream = proc_macro2::TokenStream::from($tokenstream);
                let error = syn::Error::new_spanned(tokenstream, err);
                return proc_macro::TokenStream::from(error.into_compile_error());
            }
        }
    };
}

fn include_transformed(
    input: proc_macro::TokenStream,
    transform: impl FnOnce(&Path, &Path) -> Result<(), proc_macro::TokenStream>,
) -> proc_macro::TokenStream {
    let input = expand_macro_input!(input);
    let file = parse_macro_input!(input as LitStr);

    simulate_inclusion(&file).unwrap();
    let mut src = proc_macro::Span::call_site().source_file().path();
    src.set_file_name(file.value());
    let mut dst = NamedTempFile::new().unwrap();

    if let Err(output) = transform(src.as_path(), dst.path()) {
        return output;
    }

    let output = {
        let mut buf = Vec::new();
        dst.read_to_end(&mut buf).unwrap();
        Literal::byte_string(&buf)
    };
    proc_macro::TokenStream::from(proc_macro::TokenTree::from(output))
}

macro_rules! bail {
    ($($arg:tt)*) => {{
        let msg = format!($($arg)*);
        let compile_error = quote! {
            compile_error!(#msg)
        };
        return Err(proc_macro::TokenStream::from(compile_error));
    }};
}

fn run(command: &mut Command, program: &str) -> Result<(), proc_macro::TokenStream> {
    let status = match command.status() {
        Ok(status) => status,
        Err(err) => bail!("Failed to execute {program}: {err}"),
    };
    if !status.success() {
        bail!("{program} finished with: {status}");
    }
    Ok(())
}

/// Assembles a file with [NASM] into raw binary and includes the output as a
/// reference to a byte array.
///
/// The file is located relative to the current file (similarly to how modules
/// are found). This macro will yield an expression of type `&'static [u8; N]`
/// which is the contents of the assembled file.
///
/// # Dependencies
///
/// This macro requires [NASM] to be installed.
///
/// # Examples
///
/// ```ignore
/// let boot_code = include_nasm_bin!("boot.asm");
/// ```
///
/// [NASM]: https://www.nasm.us/
#[proc_macro]
pub fn include_nasm_bin(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    include_transformed(input, |src, dst| {
        let program = "nasm";
        let mut command = Command::new(program);
        command.arg("-f").arg("bin").arg(src).arg("-o").arg(dst);
        run(&mut command, program)
    })
}
