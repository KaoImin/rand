# Rand

[![Build Status](https://travis-ci.org/rust-random/rand.svg?branch=master)](https://travis-ci.org/rust-random/rand)
[![Build Status](https://ci.appveyor.com/api/projects/status/github/rust-random/rand?svg=true)](https://ci.appveyor.com/project/rust-random/rand)
[![Crate](https://img.shields.io/crates/v/rand.svg)](https://crates.io/crates/rand)
[![Book](https://img.shields.io/badge/book-master-yellow.svg)](https://rust-random.github.io/book/)
[![API](https://img.shields.io/badge/api-master-yellow.svg)](https://rust-random.github.io/rand)
[![API](https://docs.rs/rand/badge.svg)](https://docs.rs/rand)
[![Minimum rustc version](https://img.shields.io/badge/rustc-1.32+-lightgray.svg)](https://github.com/rust-random/rand#rust-version-requirements)

A Rust library for random number generation.

Rand provides utilities to generate random numbers, to convert them to useful
types and distributions, and some randomness-related algorithms.

The core random number generation traits of Rand live in the [rand_core](
https://crates.io/crates/rand_core) crate but are also exposed here; RNG
implementations should prefer to use `rand_core` while most other users should
depend on `rand`.

Documentation:
-   [The Rust Rand Book](https://rust-random.github.io/book)
-   [API reference (master)](https://rust-random.github.io/rand)
-   [API reference (docs.rs)](https://docs.rs/rand)


## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
rand = "0.6"
```

To get started using Rand, see [The Book](https://rust-random.github.io/book).


## Versions

Rand libs have inter-dependencies and make use of the
[semver trick](https://github.com/dtolnay/semver-trick/) in order to make traits
compatible across crate versions. (This is especially important for `RngCore`
and `SeedableRng`.) A few crate releases are thus compatibility shims,
depending on the *next* lib version (e.g. `rand_core` versions `0.2.2` and
`0.3.1`). This means, for example, that `rand_core_0_4_0::SeedableRng` and
`rand_core_0_3_0::SeedableRng` are distinct, incompatible traits, which can
cause build errors. Usually, running `cargo update` is enough to fix any issues.

The Rand lib is not yet stable, however we are careful to limit breaking changes
and warn via deprecation wherever possible. Patch versions never introduce
breaking changes. The following minor versions are supported:

-   Version 0.6 was released in November 2018, redesigning the `seq` module,
    moving most PRNGs to external crates, and many small changes.
-   Version 0.5 was released in May 2018, as a major reorganisation
    (introducing `RngCore` and `rand_core`, and deprecating `Rand` and the
    previous distribution traits).
-   Version 0.4 was released in December 2017, but contained almost no breaking
    changes from the 0.3 series.

A detailed [changelog](CHANGELOG.md) is available.

When upgrading to the next minor series (especially 0.4 → 0.5), we recommend
reading the [Upgrade Guide](https://rust-random.github.io/book/update.html).

### Rust version requirements

Since version 0.7 (unreleased), Rand requires **Rustc version 1.32 or greater**.
Rand 0.5 requires Rustc 1.22 or greater while versions
0.4 and 0.3 (since approx. June 2017) require Rustc version 1.15 or
greater. Subsets of the Rand code may work with older Rust versions, but this
is not supported.

Travis CI always has a build with a pinned version of Rustc matching the oldest
supported Rust release. The current policy is that this can be updated in any
Rand release if required, but the change must be noted in the changelog.

To avoid bumping the required version unnecessarily, we use a `build.rs` script
to auto-detect the compiler version and enable certain features or change code
paths automatically. Since this makes it easy to unintentionally make use of
features requiring a more recent Rust version, we recommend testing with a
pinned version of Rustc if you require compatibility with a specific version.

## Crate Features

Rand is built with the `std` and `getrandom` features enabled by default:

-   `std` enables functionality dependent on the `std` lib and implies `alloc`
    and `getrandom`
-   `getrandom` is an optional crate, providing the code behind `rngs::OsRng`;
    the continued existance of this feature is not guaranteed so users are
    encouraged to specify `std` instead

The following optional features are available:

- `alloc` can be used instead of `std` to provide `Vec` and `Box`.
- `log` enables some logging via the `log` crate.
- `nightly` enables all unstable features (`simd_support`).
- `serde1` enables serialization for some types, via Serde version 1.
- `simd_support` enables uniform sampling of SIMD types (integers and floats).
- `stdweb` enables support for `OsRng` on `wasm32-unknown-unknown` via `stdweb`
  combined with `cargo-web`.
- `wasm-bindgen` enables support for `OsRng` on `wasm32-unknown-unknown` via
  [`wasm-bindgen`]

[`wasm-bindgen`]: https://github.com/rustwasm/wasm-bindgen

`no_std` mode is activated by setting `default-features = false`; this removes
functionality depending on `std`:

- `thread_rng()`, and `random()` are not available, as they require thread-local
  storage and an entropy source.
- Since no external entropy is available, it is not possible to create
  generators with fresh seeds using the `FromEntropy` trait (user must provide
  a seed).
- Several non-linear distributions distributions are unavailable since `exp`
  and `log` functions are not provided in `core`.
- Large parts of the `seq`-uence module are unavailable, unless the `alloc`
  feature is used (several APIs and many implementations require `Vec`).


# License

Rand is distributed under the terms of both the MIT license and the
Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT), and
[COPYRIGHT](COPYRIGHT) for details.
