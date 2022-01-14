# include-transformed

Macros for including transformed files at compile time.

The macros include a file similar to [`include_bytes!`], but transform the data before inclusion (e.g. runs an assembler).

The original purpose of this project was including assembled application processor initialization code in [libhermit-rs] using [`include_nasm_bin!`].

[`include_bytes!`]: https://doc.rust-lang.org/std/macro.include_bytes.html
[libhermit-rs]: https://github.com/hermitcore/libhermit-rs
[`include_nasm_bin!`]: https://docs.rs/include-transformed/latest/include_transformed/macro.include_nasm_bin.html

```
[dependencies]
include-transformed = "0.1"
```

# Requirements

This project depends on the unstable features `proc_macro_span` [#54725] and `proc_macro_expand` [#90765], which are only available on the nightly toolchain channel.

[#54725]: https://github.com/rust-lang/rust/issues/54725
[#90765]: https://github.com/rust-lang/rust/issues/90765


## License

This project is licensed under either of

* [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0) ([LICENSE-APACHE](LICENSE-APACHE))

* [MIT License](https://opensource.org/licenses/MIT) ([LICENSE-MIT](LICENSE-MIT))

at your option.


## Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in include-transformed by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
