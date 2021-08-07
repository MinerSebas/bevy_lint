# Bevy Lint

## What is Bevy Lint?

This crates provides Lints for Bevy Code using [dylint](https://github.com/trailofbits/dylint).

## How to run Lints

Add this to your Cargo.toml:

```toml
[workspace.metadata.dylint]
libraries = [
    { git = "https://github.com/MinerSebas/bevy_lint", branch = "main" },
]
```

Instead of a `branch`, you can also provide a `tag` or a `rev` (revision)

Afterwards you need to run these commans:

```sh
cargo install cargo-dylint dylint-link    # Only neccesary once
cargo dylint bevy_lint
```

If you are using the MSVC Toolchain, you will need to manualy build dylint from Source, until [https://github.com/trailofbits/dylint/pull/45](https://github.com/trailofbits/dylint/pull/45) is merged and released.

```sh
git clone https://github.com/MinerSebas/dylint
cd dylint
git checkout linker
cargo install --path cargo-dylint
cargo install --path dylint-link
```

## Lint Creation

A Lint is created by implementing the [LateLintPass](https://doc.rust-lang.org/stable/nightly-rustc/rustc_lint/trait.LateLintPass.html) trait and adding to the `register_lints` function.

When creating a UI Test, add the Test as an Example to the [Cargo.toml](Cargo.toml).
Also make sure that your `.stderr` File uses `LF` Line-endings and not `CRLF`, as otherwise the Test will fail without any explanation.

For more Resources you can take a look at the [dylint resources](https://github.com/trailofbits/dylint#resources).

## License

bevy_lint is free and open source! All code in this repository is dual-licensed under either:

* MIT License ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))

at your option. This means you can select the license you prefer! This dual-licensing approach is the de-facto standard in the Rust ecosystem.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
