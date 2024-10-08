mod command;
mod manifest;

use std::fs;
use std::io::{BufReader, Result, Write};

use std::path::Path;
use std::{collections::HashMap, fs::File};

use clap::Parser;
use command::Cli;
use manifest::*;
use serde::{Deserialize, Serialize};
use zip::write::SimpleFileOptions;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    pack: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    if let Some(command) = cli.command {
        match command {
            command::Command::Check { pack } => todo!("implement check"),
            command::Command::Run { pack } => {
                let f = File::open("pack.yaml")?;
                let reader = BufReader::new(f);
                let manifest: Manifest = serde_yml::from_reader(reader).unwrap();
                let pack = manifest.packs.get(pack.unwrap().as_str()).unwrap();
                match doit(pack) {
                    Ok(_) => println!("File written to {0}", &pack.filename),
                    Err(e) => println!("Error: {e:?}"),
                }
            }
        }
    } else {
        todo!("print help");
    }

    Ok(())
}

fn print_yaml<T>(t: &T)
where
    T: Serialize,
{
    let yaml = serde_yml::to_string(t).unwrap();
    println!("Serialized YAML:\n{}", yaml);
}

fn doit(package: &Package) -> zip::result::ZipResult<()> {
    let path = std::path::Path::new(&package.filename);
    let file = std::fs::File::create(path).unwrap();
    let mut zip = zip::ZipWriter::new(file);
    for e in &package.entries {
        let options = SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Stored)
            .unix_permissions(0o755);
        zip.add_directory_from_path(&e.dest_dir, options)?;
        for f in &e.files {
            match f {
                FileMapping::Source(s) => {
                    zip.start_file(s.as_str(), options)?;
                    let content = fs::read(s.as_str())?;
                    zip.write_all(&content)?;
                }
                FileMapping::SourceWithDestination { src, dest } => {
                    zip.start_file(dest.as_str(), options)?;
                    let content = fs::read(src.as_str())?;
                    zip.write_all(&content)?;
                }
            }
        }
    }

    zip.finish()?;
    Ok(())
}
