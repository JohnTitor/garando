Rust syntax backport
====================

[![Build Status](https://api.travis-ci.org/serde-rs/garando.png?branch=master)](https://travis-ci.org/serde-rs/garando)
[![Latest Version](https://img.shields.io/crates/v/garando_syntax.svg)](https://crates.io/crates/garando_syntax)

This repository contains a backport of the following unstable crates from the
Rust compiler.

- [`libsyntax`] => [`garando_syntax`]
- [`libsyntax_pos`] => [`garando_pos`]
- [`librustc_errors`] => [`garando_errors`]

[`libsyntax`]: https://github.com/rust-lang/rust/tree/master/src/libsyntax
[`garando_syntax`]: https://docs.rs/garando_syntax
[`libsyntax_pos`]: https://github.com/rust-lang/rust/tree/master/src/libsyntax_pos
[`garando_pos`]: https://docs.rs/garando_pos
[`librustc_errors`]: https://github.com/rust-lang/rust/tree/master/src/librustc_errors
[`garando_errors`]: https://docs.rs/garando_errors

The backported code compiles on the most recent stable release of Rust.

## License

garando is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in garando by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
