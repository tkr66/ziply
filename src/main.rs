mod command;
mod manifest;
mod pack;

use std::io::Result;

use std::path::Path;

use clap::Parser;
use command::Cli;
use command::Command;

fn main() -> Result<()> {
    let cli = Cli::parse();
    let manifest = match cli.file {
        Some(f) => manifest::read(&f)?,
        None => manifest::read(Path::new("ziply.yaml"))?,
    };
    match cli.command {
        Command::Check { pack } => match pack {
            Some(p) => pack::check(&manifest, &p)?,
            None => pack::check_all(&manifest)?,
        },
        Command::Run { pack } => match pack {
            Some(p) => pack::run(&manifest, &p)?,
            None => pack::run_all(&manifest)?,
        },
    }

    Ok(())
}
