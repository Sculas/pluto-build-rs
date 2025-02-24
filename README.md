# Pluto for Rust

A Rust library for compiling and linking [Pluto](https://pluto-lang.org/) into your Rust project.

**Currently, Windows and Linux are supported. MacOS support exists but is untested.**

## Usage

Add the following to your `Cargo.toml`:

```toml
[dependencies]
# Add the `pluto-ffi` crate to your project if you want to use any of Pluto's standard libraries.
pluto-ffi = "0.1.0"

[build-dependencies]
# pluto-build's version is suffixed with the Pluto version it was built against.
# It's recommended to pin this to the exact version of Pluto you want to use.
pluto-build = "=0.2.0-0.10.4"
```

Then, in your `build.rs` file, add the following:

```rust
use pluto_build as pluto;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    pluto::Build::new()
        // All options are prefixed with `.opt_` and are documented.
        // .opt_ilp_enabled() // enables Infinite Loop Protection
        .compile();
}
```

This will compile Pluto and link it statically into your project. You can then use it with `mlua` to get a safe interface to Pluto:

> https://github.com/mlua-rs/mlua/pull/529 must be merged before this will work.

```toml
[dependencies]
mlua = { version = "0.10", features = ["lua54", "external"] }
```

To be able to use Pluto's standard libraries, you must add the following after you've created your `Lua` instance:

```rust
let lua = mlua::Lua::new();
pluto_ffi::load_libraries!(&lua)?;
```

Or, if you want to load only specific libraries:

```rust
let lua = mlua::Lua::new();
pluto_ffi::load_libraries!(&lua, &[pluto_ffi::PlutoLibrary::Base64])?;
```

## Updating Pluto

- Download the latest version of the Pluto source code from the repository.
- Update the patches in the `patches` directory to match the new Pluto version. You may need to fix the patch if something has changed.
- Apply the patches using the `patch.ps1` script in the root directory.
- Update the `pluto-build` version suffix in the `Cargo.toml` file to use the new Pluto version.

## License

This project is licensed under the MIT license and includes code from the [Pluto](https://github.com/PlutoLang/Pluto)
project, which is also licensed under the MIT license. Both licenses can be found in the root and Pluto source directory.
