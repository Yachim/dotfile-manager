#[path = "../import.rs"]
mod import;
#[path = "list.rs"]
mod list;
#[path = "../pull.rs"]
mod pull;

use clap::{Arg, ArgAction, ArgMatches, Command};
use import::import;
use list::list_templates;
use pull::{pull, pull_all};
use std::cfg;

/// Get arguments from Clap
fn arguments() -> ArgMatches {
    Command::new("dman")
        .about("Manage your dotfiles")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .author("Adamekka")
        .subcommand(
            Command::new("import")
                .about("Import a template")
                .arg(
                    Arg::new("name")
                        .short('n')
                        .long("name")
                        .action(ArgAction::Append)
                        .required(true),
                )
                .arg(
                    Arg::new("path")
                        .short('p')
                        .long("path")
                        .action(ArgAction::Append)
                        .required(true),
                )
                .arg(
                    Arg::new("git-path")
                        .short('g')
                        .long("git-path")
                        .action(ArgAction::Append)
                        .required(true),
                ),
        )
        .subcommand(Command::new("list").about("List imported templates"))
        .subcommand(
            Command::new("pull")
                .about("Pull changes from Git repo")
                .arg(
                    Arg::new("name")
                        .short('n')
                        .long("name")
                        .action(ArgAction::Append),
                )
                .arg(
                    Arg::new("path")
                        .short('p')
                        .long("path")
                        .action(ArgAction::Append),
                )
                .arg(
                    Arg::new("git-path")
                        .short('g')
                        .long("git-path")
                        .action(ArgAction::Append),
                ),
        )
        .subcommand(Command::new("pull-all").about("Pull all changes from all Git repo(s)"))
        .subcommand(
            Command::new("push")
                .about("Push changes to Git repo")
                .arg(
                    Arg::new("name")
                        .short('n')
                        .long("name")
                        .action(ArgAction::Append),
                )
                .arg(
                    Arg::new("path")
                        .short('p')
                        .long("path")
                        .action(ArgAction::Append),
                )
                .arg(
                    Arg::new("git-path")
                        .short('g')
                        .long("git-path")
                        .action(ArgAction::Append),
                ),
        )
        .subcommand(Command::new("push-all").about("Push all changes to all Git repo(s)"))
        .get_matches()
}

/// Match arguments: import, pull, push, ...
/// Then pass them to according function with their parameters
pub fn match_args() {
    let args = arguments();
    match args.subcommand() {
        Some(("import", _set_matches)) => {
            let (name, path, git_path) = match_subcmd_flags("import");
            import(name, path, git_path);
        }

        Some(("list", _set_matches)) => {
            list_templates();
        }

        Some(("pull", _set_matches)) => {
            check_if_enough_flags("pull");
            let (name, path, git_path) = match_subcmd_flags("pull");
            pull(name, path, git_path);
        }

        Some(("pull-all", _set_matches)) => {
            pull_all();
        }

        Some(("push", _set_matches)) => {
            check_if_enough_flags("push");
            let (_name, _path, _git_path) = match_subcmd_flags("push");
            todo!("push");
        }

        Some(("push-all", _set_matches)) => {
            todo!("push-all");
        }

        _ => unreachable!(),
    }
}

/// Match subcommand flags: -n, -p, -g; --name, --path, --git-path
fn match_subcmd_flags(
    cmd: &str,
) -> (
    Option<std::string::String>,
    Option<std::string::String>,
    Option<std::string::String>,
) {
    let args = arguments();

    let mut name: Option<String> = None;
    let mut path: Option<String> = None;
    let mut git_path: Option<String> = None;

    if let Some(arg_match) = args.subcommand_matches(cmd) {
        if arg_match.get_one::<String>("name").is_some() {
            name = Some(arg_match.get_one::<String>("name").unwrap().to_string());
        }

        if arg_match.get_one::<String>("path").is_some() {
            path = Some(arg_match.get_one::<String>("path").unwrap().to_string());
        }

        if arg_match.get_one::<String>("git-path").is_some() {
            git_path = Some(arg_match.get_one::<String>("git-path").unwrap().to_string());
        }
    } else {
        // this never gets called
        panic!("Clap somehow screwed up");
    }

    #[cfg(debug_assertions)]
    {
        println!("{:?}, {:?}, {:?}", name, path, git_path);
    }

    (name, path, git_path)
}

/// Check if at least 1 flag is present
fn check_if_enough_flags(cmd: &str) {
    let args = arguments();

    if let Some(arg_match) = args.subcommand_matches(cmd) {
        if !(arg_match.get_one::<String>("name").is_some()
            || arg_match.get_one::<String>("path").is_some()
            || arg_match.get_one::<String>("git-path").is_some())
        {
            panic!("At least 1 flag is required");
        }
    }
}
