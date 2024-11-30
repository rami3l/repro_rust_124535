# repro_rust_124535

Checking this repo currently gives the following warning:

``` console
> cargo check
    Checking repro_rust_124535 v0.1.0 (/path/censored)
warning: cannot find macro `doc_self` in this scope
 --> src/pm/dnf.rs:1:10
  |
1 | #![doc = doc_self!()]
  |          ^^^^^^^^
  |
  = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
  = note: for more information, see issue #124535 <https://github.com/rust-lang/rust/issues/124535>
  = help: import `macro_rules` with `use` to make it callable above its definition
  = note: `#[warn(out_of_scope_macro_calls)]` on by default

warning: `repro_rust_124535` (lib) generated 1 warning
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.03s
```

Reproduction environment:

```console
> uname -rvmps
Darwin 24.1.0 Darwin Kernel Version 24.1.0: Thu Oct 10 21:03:15 PDT 2024; root:xnu-11215.41.3~2/RELEASE_ARM64_T6000 arm64 arm

> rustup --version
rustup 1.28.0 :: 1.27.1+500 (b6fdf2fa8 2024-11-25)
info: This is the version for the rustup toolchain manager, not the rustc compiler.
info: The currently active `rustc` version is `rustc 1.83.0 (90b35a623 2024-11-26)`

> cargo --version
cargo 1.83.0 (5ffbef321 2024-10-29)
```
