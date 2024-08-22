use std::io::{BufReader, Result, Write};
use std::path::Path;
use std::{collections::HashMap, fs::File};

use clap::Parser;
use serde::{Deserialize, Serialize};
use zip::write::SimpleFileOptions;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    pack: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct Manifest {
    packs: HashMap<String, Package>,
}

#[derive(Deserialize, Serialize, Debug)]
struct Package {
    filename: String,
    entries: Vec<Entry>,
}

#[derive(Deserialize, Serialize, Debug)]
struct Entry {
    src: String,
    dest: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let f = File::open("pack.yaml")?;
    let reader = BufReader::new(f);
    let manifest: Manifest = serde_yml::from_reader(reader).unwrap();
    let pack = manifest.packs.get(&args.pack).unwrap();
    match doit(pack) {
        Ok(_) => println!("File written to {0}", &pack.filename),
        Err(e) => println!("Error: {e:?}"),
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
        let dest_path = Path::new(&e.dest);
        zip.add_directory(
            dest_path.parent().unwrap().to_str().unwrap(),
            SimpleFileOptions::default(),
        )?;
        let options = SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Stored)
            .unix_permissions(0o755);
        zip.start_file(dest_path.to_str().unwrap(), options)?;
        //TODO write the actual contents
        zip.write_all(b"Hello, World!\n")?;
    }

    zip.finish()?;
    Ok(())
}
