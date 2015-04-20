#![deny(warnings)]

extern crate cargo;

use std::process::Output;

use cargo::ops::{ExecEngine, CommandPrototype};
use cargo::util::{ProcessError, ProcessBuilder};

type ExecResult = Result<Option<Output>, ProcessError>;

#[derive(Debug)]
pub struct LinkArgsEngine {
    pub pkg_name: String,
    pub link_args: Option<String>,
    pub print_link_args: bool,
}

impl ExecEngine for LinkArgsEngine {
    fn exec(&self, command: CommandPrototype) -> Result<(), ProcessError> {
        exec_append_linkargs(command, false, self).map(|_| ())
    }

    fn exec_with_output(&self, command: CommandPrototype) -> Result<Output, ProcessError> {
        exec_append_linkargs(command, true, self).map(|a| a.unwrap())
    }
}

fn exec_append_linkargs(command: CommandPrototype, with_output: bool,
                        engine: &LinkArgsEngine) -> ExecResult {
    let command = append_linkargs(command, engine);
    execute(command.into_process_builder(), with_output)
}

fn append_linkargs(mut command: CommandPrototype, engine: &LinkArgsEngine) -> CommandPrototype {
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

    command
}

fn execute(process: ProcessBuilder, with_output: bool) -> ExecResult {
    if with_output {
        process.exec_with_output().map(|o| Some(o))
    } else {
        process.exec().map(|_| None)
    }
}

#[cfg(test)]
mod tests {
    use cargo::ops::{CommandPrototype, CommandType};
    use {append_linkargs, LinkArgsEngine};

    fn cmd_and_engine(name: &str) -> (CommandPrototype, LinkArgsEngine) {
        let mut cmd = CommandPrototype::new(CommandType::Rustc).unwrap();
        cmd.args(&["--crate-name", "test_bin", "--crate-type", "bin"]);
        let engine = LinkArgsEngine {
            pkg_name: name.to_string(),
            link_args: Some("-first --num 2".to_string()),
            print_link_args: true,
        };
        (cmd, engine)
    }

    #[test]
    fn do_append_linkargs() {
        let (cmd, engine) = cmd_and_engine("test_bin");

        let cmd = append_linkargs(cmd, &engine);
        assert!(cmd.get_args().contains(&From::from("link-args=-first --num 2")));
        assert!(cmd.get_args().contains(&From::from("print-link-args")))
    }

    #[test]
    fn not_append_linkargs() {
        let (cmd, engine) = cmd_and_engine("test_lib");

        let cmd = append_linkargs(cmd, &engine);
        assert!(!cmd.get_args().contains(&From::from("link-args=-first --num 2")));
        assert!(!cmd.get_args().contains(&From::from("print-link-args")))
    }
}
