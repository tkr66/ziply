use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser, PartialEq, Debug)]
#[command(version, about)]
pub struct Cli {
    #[arg(short, long)]
    pub file: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, PartialEq, Debug)]
pub enum Command {
    Check {
        #[arg(short, long)]
        pack: Option<String>,
    },
    Run {
        #[arg(short, long)]
        pack: Option<String>,
    },
}

#[test]
fn implicit_file() {
    let x = Cli {
        file: None,
        command: Command::Check {
            pack: Some("pack1".to_string()),
        },
    };
    let y = Cli::try_parse_from(["test", "check", "-p", "pack1"]).unwrap();
    assert_eq!(x, y);
}

#[test]
fn explicit_file() {
    let x = Cli {
        file: Some("ziply.yaml".into()),
        command: Command::Check { pack: None },
    };
    let y = Cli::try_parse_from(["test", "-f", "ziply.yaml", "check"]).unwrap();
    assert_eq!(x, y);
}

#[test]
fn explicit_pack() {
    let x = Cli {
        file: None,
        command: Command::Check {
            pack: Some("pack1".to_string()),
        },
    };
    let y = Cli::try_parse_from(["test", "check", "-p", "pack1"]).unwrap();
    assert_eq!(x, y);
}
