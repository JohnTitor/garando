garando, Rust syntax backport
====================

[![CI](https://github.com/JohnTitor/garando/actions/workflows/ci.yml/badge.svg)](https://github.com/JohnTitor/garando/actions/workflows/ci.yml) [![Latest Version](https://img.shields.io/crates/v/garando_syntax.svg)](https://crates.io/crates/garando_syntax)

This repository contains a backport of the following unstable crates from the
Rust compiler.

- `libsyntax` => [`garando_syntax`]
- `libsyntax_pos` => [`garando_pos`]
- `librustc_errors` => [`garando_errors`]

[`garando_syntax`]: https://docs.rs/garando_syntax
[`garando_pos`]: https://docs.rs/garando_pos
[`garando_errors`]: https://docs.rs/garando_errors

The code compiles on the most recent stable release of Rust.

## License

garando is licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   https://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or
   https://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in garando by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
