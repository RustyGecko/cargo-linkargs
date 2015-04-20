#![allow(warnings)]

extern crate cargo;

use std::process::{Output, Command, Stdio};
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::{self, BufReader, BufRead, BufWriter, Write, copy};
// use std::os::self_exe_path;
use std::mem::swap;

use cargo::ops::{ExecEngine, CommandPrototype, CommandType};
use cargo::util::{self, ProcessError, ProcessBuilder};
use cargo::core::PackageIdSpec;

type ExecResult = Result<Option<Output>, ProcessError>;

#[derive(Debug)]
pub struct LinkArgsEngine {
    pub pkg_name: String,
    pub link_args: Option<String>,
    pub print_link_args: bool,
}

impl ExecEngine for LinkArgsEngine {
    fn exec(&self, command: CommandPrototype) -> Result<(), ProcessError> {
        append_linkargs(command, false, self).map(|_| ())
    }

    fn exec_with_output(&self, command: CommandPrototype) -> Result<Output, ProcessError> {
        append_linkargs(command, true, self).map(|a| a.unwrap())
    }
}

fn append_linkargs(mut command: CommandPrototype, with_output: bool,
                   engine: &LinkArgsEngine) -> ExecResult {

    let name_matches = command.get_args().windows(2).find(|&args| {
        args[0].to_str() == Some("--crate-name") &&
        args[1].to_str() == Some(&engine.pkg_name)
    }).is_some();

    let is_binary = command.get_args().windows(2).find(|&args| {
        args[0].to_str() == Some("--crate-type") &&
        args[1].to_str() == Some("bin")
    }).is_some();

    if is_binary && name_matches {
        if engine.link_args.is_some() {
            command.arg("-C").arg(&format!("link-args={}", engine.link_args.as_ref().unwrap()));
        }

        if engine.print_link_args {
            command.arg("-Z").arg("print-link-args");
        }
    }

    execute(command.into_process_builder(), with_output)
}

fn execute(process: ProcessBuilder, with_output: bool) -> ExecResult {
    if with_output {
        process.exec_with_output().map(|o| Some(o))
    } else {
        process.exec().map(|_| None)
    }
}
