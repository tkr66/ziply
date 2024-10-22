mod command;
mod manifest;
mod pack;

use std::io::Result;

use std::path::PathBuf;

use clap::Parser;
use command::Cli;
use command::Command;

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Check { pack } => match pack {
            Some(p) => {
                let file = cli.file.unwrap_or(PathBuf::from("ziply.yaml"));
                let m = manifest::read(&file)?;
                pack::check(&m, &p)?;
            }
            None => {
                let file = cli.file.unwrap_or(PathBuf::from("ziply.yaml"));
                let m = manifest::read(&file)?;
                pack::check_all(&m)?;
            }
        },
        Command::Run { pack } => match pack {
            Some(p) => {
                let file = cli.file.unwrap_or(PathBuf::from("ziply.yaml"));
                let m = manifest::read(&file)?;
                pack::run(&m, &p)?
            }
            None => {
                let file = cli.file.unwrap_or(PathBuf::from("ziply.yaml"));
                let m = manifest::read(&file)?;
                pack::run_all(&m)?
            }
        },
    }

    Ok(())
}
