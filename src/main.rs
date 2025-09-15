use anyhow::Result;
use clap::{Parser, ValueEnum};

use std::env;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// The variable to operate on, like PATH or MANPATH
    variable: String,
    /// Whether to prepend or append the paths
    #[arg(value_enum)]
    action: Action,
    /// Keep the existing or force the requested precedence
    #[arg(value_enum)]
    precedence: Precedence,
    /// The paths to prepend or append, in the order they should appear
    paths: Vec<PathBuf>,
}

#[derive(Clone, Debug, PartialEq, Eq, ValueEnum)]
enum Action {
    Prepend,
    Append,
}

#[derive(Clone, Debug, PartialEq, Eq, ValueEnum)]
enum Precedence {
    Force,
    Keep,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let mut paths =
        env::split_paths(&env::var(args.variable).unwrap_or_default()).collect::<Vec<_>>();

    paths.retain(|p| !p.as_os_str().is_empty());

    if args.precedence == Precedence::Force {
        for path in &args.paths {
            paths.retain(|p| p != path);
        }
    }

    let mut args_paths = args.paths;
    if args.action == Action::Prepend {
        args_paths.reverse();
    }

    for path in args_paths {
        if args.precedence == Precedence::Keep && paths.contains(&path) {
            continue;
        }

        match args.action {
            Action::Prepend => paths.insert(0, path),
            Action::Append => paths.push(path),
        }
    }

    #[cfg(target_family = "unix")]
    const SEPARATOR: char = ':';
    #[cfg(target_family = "windows")]
    const SEPARATOR: char = ';';

    let result = env::join_paths(paths)?.to_string_lossy().trim_matches(SEPARATOR).to_string();
    
    println!("{}", result);
    
    Ok(())
}
