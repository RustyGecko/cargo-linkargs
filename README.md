# cargo-linkargs
[![Build Status](https://travis-ci.org/RustyGecko/cargo-linkargs.svg)](https://travis-ci.org/RustyGecko/cargo-linkargs)
### Note: This subcommand is depcrated in favor of the built in `cargo rustc` command
A Cargo subcommand to apply the `-C link-args="..."` rustc codegen option to the final binary built with Cargo.

`cargo-linkargs` executes a build on a normal Cargo project just like the `cargo build` command.
It extends the `build` commands with an additional `<args>` parameter that will be passed further
on to the linker, and an option to print the link args.

Multiple linker arguments need to be put in a string, and seperated by spaces like so: `"many link args"`.

Note that both of these parameters will *only* be passed on as arguments to the *final* build of the library.


## Installation
Currently, `cargo-linkargs` depends directly on the Cargo source code, so make sure that
you have everything set up correctly in order to [compile cargo](https://github.com/rust-lang/cargo/#compiling-cargo).

Build the project and make sure the `cargo-linkargs` binary is available in the path.

For example:
```bash
$ cargo build --release
$ export PATH=$PATH:`pwd`/target/release
```
## Usage
Once the `cargo-linkargs` binary is in the path, it can be used like any other Cargo subcommand:

```
Usage:
    cargo linkargs [options] [<args>]
Options:
    -h, --help               Print this message
    -p SPEC, --package SPEC  Package to build
    -j N, --jobs N           The number of jobs to run in parallel
    --lib                    Build only lib (if present in package)
    --build-examples         Build all examples (if present in package)
    --example NAME           Name of the example to build
    --release                Build artifacts in release mode, with optimizations
    --features FEATURES      Space-separated list of features to also build
    --no-default-features    Do not build the `default` feature
    --target TRIPLE          Build for the target triple
    --manifest-path PATH     Path to the manifest to compile
    -v, --verbose            Use verbose output
    --print-link-args        Print the arguments passed to the linker for the final binary
```

## Example
```bash
$ cargo linkargs "--link-with foo -Tpath/to/some.ld"
```
