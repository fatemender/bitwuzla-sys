[![crates.io](https://img.shields.io/crates/v/bitwuzla-sys.svg)](https://crates.io/crates/bitwuzla-sys)
[![docs.rs](https://img.shields.io/docsrs/bitwuzla-sys)](https://docs.rs/bitwuzla-sys)

# bitwuzla-sys

This Rust crate provides low-level bindings for the [Bitwuzla] SMT solver,
version 0.7.0.

[Bitwuzla]: https://bitwuzla.github.io/

## Installation

### Using shared `bitwuzla` library

Compile `bitwuzla` as a shared library and install it.  Then add this crate
to your `Cargo.toml`:

```toml
[dependencies]
bitwuzla-sys = "0.7"
```

### Using vendored static `bitwuzla` library

This is possible on UNIX-like targets only.  Add this crate to your `Cargo.toml`
with the `vendor-cadical` feature enabled:

```toml
[dependencies]
bitwuzla-sys = { version = "0.7", features = ["vendor-cadical"] }
```

Enabling `vendor-cadical` will automatically build a static `bitwuzla` library
and link against it.  Currently this uses the CaDiCaL SAT solver.

In order for the build to succeed, you'll need to install some tools on your
build host; for a Debian-based distribution `build-essential`, `git`, `m4`,
and `meson` should be sufficient.

## License

This crate is licensed under the [MIT license].

[MIT license]: LICENSE
