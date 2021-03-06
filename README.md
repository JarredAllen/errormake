errormake
=========

[![Crates.io Page](https://img.shields.io/crates/v/errormake.svg)][crates]
[![Docs.rs Page](https://docs.rs/errormake/badge.svg)][docs]
![License](https://img.shields.io/crates/l/errormake.svg)
[![Travis CI Status](https://api.travis-ci.com/JarredAllen/errormake.svg?branch=master)][travis]

A macro for automatically creating Error structs in Rust

- [Source code][repo]
- [Documentation][docs]
- [Crates.io page][crates]

## Usage

To use in your rust crate, add this to your `Cargo.toml`:
```toml
[dependencies]
errormake = "0.2.1"
```

and add this in the module where you want to use it
```rust
use errormake::errormake;
```

For the full documentation on how to use this crate, consult this
crate's [documentation on docs.rs][docs]

## Contributing

To contribute to this project, see [CONTRIBUTING.md][contributing].

[contributing]: ./CONTRIBUTING.md
[crates]: https://crates.io/crates/errormake
[docs]: https://docs.rs/errormake
[repo]: https://github.com/JarredAllen/errormake
[travis]: https://travis-ci.com/github/JarredAllen/errormake
