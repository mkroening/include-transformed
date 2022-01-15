# include-transformed

[<img alt="github" src="https://img.shields.io/badge/github-mkroening/include--transformed-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/mkroening/include-transformed)
[<img alt="crates.io" src="https://img.shields.io/crates/v/include-transformed.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/include-transformed)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-include--transformed-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K" height="20">](https://docs.rs/include-transformed)

Macros for including transformed files at compile time.

The macros include a file similar to [`include_bytes!`], but transform the data before inclusion. The transformation could be, for example, assembling assembly code to machine code ([`include_nasm_bin!`]).

The original purpose of this project was including assembled application processor initialization code in [libhermit-rs] using [`include_nasm_bin!`].

[`include_bytes!`]: https://doc.rust-lang.org/std/macro.include_bytes.html
[libhermit-rs]: https://github.com/hermitcore/libhermit-rs
[`include_nasm_bin!`]: https://docs.rs/include-transformed/latest/include_transformed/macro.include_nasm_bin.html

```
[dependencies]
include-transformed = "0.2"
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
