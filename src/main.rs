mod command;
mod manifest;
mod pack;

use std::io::Result;

use std::path::PathBuf;

use clap::Parser;
use command::Cli;
use manifest::*;

fn main() -> Result<()> {
    let cli = Cli::parse();
    if let Some(command) = cli.command {
        match command {
            command::Command::Check { pack } => match pack {
                Some(p) => {
                    let file = cli.file.unwrap_or(PathBuf::from("pack.yaml"));
                    let m = manifest::read(&file)?;
                    pack::check(&m, &p)?;
                }
                None => {
                    let file = cli.file.unwrap_or(PathBuf::from("pack.yaml"));
                    let m = manifest::read(&file)?;
                    pack::check_all(&m)?;
                }
            },
            command::Command::Run { pack } => match pack {
                Some(p) => {
                    let file = cli.file.unwrap_or(PathBuf::from("pack.yaml"));
                    let m = manifest::read(&file)?;
                    pack::run(&m, &p)?
                }
                None => {
                    let file = cli.file.unwrap_or(PathBuf::from("pack.yaml"));
                    let m = manifest::read(&file)?;
                    pack::run_all(&m)?
                }
            },
        }
    }

    Ok(())
}
