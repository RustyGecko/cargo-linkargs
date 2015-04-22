#![deny(warnings)]
#![feature(plugin)]
#![plugin(docopt_macros)]

extern crate rustc_serialize;
extern crate docopt;
extern crate cargo;
extern crate cargo_linkargs;

use std::sync::Arc;
use std::path::PathBuf;

use docopt::Docopt;

use cargo::core::{MultiShell, Source};
use cargo::ops::{self, CompileFilter, CompileOptions, ExecEngine};
use cargo::sources::PathSource;
use cargo::util::{CliError, Config};
use cargo::util::important_paths::find_root_manifest_for_cwd;

use cargo_linkargs::LinkArgsEngine;

docopt!(Options derive Debug, "
Compile a local package and all of its dependencies, providing link arguments to the final binary

Usage:
    cargo linkargs [options] <args>

Options:
    -h, --help               Print this message
    -p SPEC, --package SPEC  Package to build
    -j N, --jobs N           The number of jobs to run in parallel
    --lib                    Build only lib (if present in package)
    --release                Build artifacts in release mode, with optimizations
    --features FEATURES      Space-separated list of features to also build
    --no-default-features    Do not build the `default` feature
    --target TRIPLE          Build for the target triple
    --manifest-path PATH     Path to the manifest to compile
    -v, --verbose            Use verbose output
    --print-link-args        Print the arguments passed to the linker for the final binary

If the --package argument is given, then SPEC is a package id specification
which indicates which package should be built. If it is not given, then the
current package is built. For more information on SPEC and its format, see the
`cargo help pkgid` command.

Compilation can be configured via the use of profiles which are configured in
the manifest. The default profile for this command is `dev`, but passing
the --release flag will use the `release` profile instead.
",
arg_args: Option<String>,
flag_package: Option<String>,
flag_jobs: Option<u32>,
flag_features: Vec<String>,
flag_target: Option<String>,
flag_manifest_path: Option<String>,
);

fn get_target_names(root: &PathBuf, shell: &mut MultiShell) -> Vec<String> {
    let config = Config::new(shell).unwrap();
    let mut source = PathSource::for_path(root.parent().unwrap(),
                                           &config).unwrap();
    let _ = source.update();
    let package = source.root_package().unwrap();
    package.targets().iter()
        .filter(|t| t.is_bin() || t.is_example())
        .map(|t| t.name().to_string())
        .collect()
}

fn main() {
    let options: Options = Options::docopt()
                                    .decode()
                                    .unwrap_or_else(|e| e.exit());
    let mut shell = cargo::shell(options.flag_verbose);
    let root = find_root_manifest_for_cwd(options.flag_manifest_path).unwrap();
    let spec = options.flag_package.as_ref().map(|s| &s[..]);

    let targets = get_target_names(&root, &mut shell);

    let result: Result<Option<()>, CliError> = {
        let config = Config::new(&mut shell).unwrap();

        let filter = match options.flag_lib {
            true => CompileFilter::Only {
                lib: true, bins: &[], examples: &[], benches: &[], tests: &[]
            },
            false => CompileFilter::Everything,
        };


        let engine = LinkArgsEngine {
            targets: targets,
            link_args: options.arg_args.clone(),
            print_link_args: options.flag_print_link_args,
        };

        let mut opts = CompileOptions {
            config: &config,
            jobs: options.flag_jobs,
            target: options.flag_target.as_ref().map(|t| &t[..]),
            features: &options.flag_features,
            no_default_features: options.flag_no_default_features,
            spec: spec,
            filter: filter,
            exec_engine: Some(Arc::new(Box::new(engine) as Box<ExecEngine>)),
            release: options.flag_release,
            mode: ops::CompileMode::Build,
        };

        ops::compile(&root, &mut opts).map(|_| None).map_err(|err| {
            cargo::util::CliError::from_boxed(err, 101)
        })
    };

    cargo::process_executed(result, &mut shell);
}
