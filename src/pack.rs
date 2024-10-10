use std::{
    collections::HashMap,
    fs::{self, File, FileType},
    io::{self, BufReader, Error, ErrorKind, Write},
    path::Path,
};

use zip::write::SimpleFileOptions;

use crate::{FileMapping, FilesWithDestination, Manifest, Package};

pub fn check(manifest: &Manifest, name: &str) -> Result<(), Error> {
    let pack = manifest.packs.get(name).expect("key not found");
    if let Some(parent) = Path::parent(Path::new(pack.filename.as_str())) {
        if parent.to_str().unwrap() != "" && !parent.exists() {
            return Err(Error::new(
                ErrorKind::NotFound,
                "The output directory does not exist",
            ));
        }
    }
    for entry in &pack.entries {
        for file in &entry.files {
            match file {
                crate::FileMapping::Source(src) => {
                    if !Path::new(src).exists() {
                        return Err(Error::new(ErrorKind::NotFound, "The source does not exist"));
                    }
                }
                crate::FileMapping::SourceWithDestination { src, .. } => {
                    if !Path::new(src).exists() {
                        return Err(Error::new(ErrorKind::NotFound, "The source does not exist"));
                    }
                }
            }
        }
    }

    Ok(())
}

pub fn check_all(manifest: &Manifest) -> Result<(), Error> {
    for k in manifest.packs.keys() {
        check(manifest, k.as_str())?;
    }

    Ok(())
}

pub fn run(manifest: &Manifest, name: &str) -> Result<(), Error> {
    let package = manifest
        .packs
        .get(name)
        .expect("The specified pack does not exist");
    let path = std::path::Path::new(&package.filename);
    let file = std::fs::File::create(path).unwrap();
    let mut zip = zip::ZipWriter::new(file);
    for e in &package.entries {
        let dir_option = SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Stored)
            .unix_permissions(0o755);
        let file_option = SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Stored)
            .unix_permissions(0o644);
        zip.add_directory_from_path(&e.dest_dir, dir_option)?;
        for f in &e.files {
            match f {
                FileMapping::Source(s) => {
                    zip.start_file(s.as_str(), file_option)?;
                    let content = fs::read(s.as_str())?;
                    zip.write_all(&content)?;
                }
                FileMapping::SourceWithDestination { src, dest } => {
                    zip.start_file(dest.as_str(), file_option)?;
                    let content = fs::read(src.as_str())?;
                    zip.write_all(&content)?;
                }
            }
        }
    }

    zip.finish()?;

    Ok(())
}

pub fn run_all(manifest: &Manifest) -> Result<(), Error> {
    for k in manifest.packs.keys() {
        run(manifest, k)?;
    }

    Ok(())
}

#[test]
fn check_not_found_output_dir() {
    let name = String::from("test");
    let mut map: HashMap<String, Package> = HashMap::new();
    let package = Package::new("/dir/not/found/test.zip".to_string(), Vec::new());
    map.insert(name.clone(), package);
    let manifest = Manifest::new(map);
    assert!(check(&manifest, name.as_str()).is_err_and(|e| e.kind() == ErrorKind::NotFound));
}

#[test]
fn check_not_found_source() {
    let name = String::from("test");
    let mut map: HashMap<String, Package> = HashMap::new();
    let package = Package::new(
        "test.zip".to_string(),
        vec![FilesWithDestination {
            dest_dir: "".to_string(),
            files: vec![FileMapping::Source("src/not/found".to_string())],
        }],
    );
    map.insert(name.clone(), package);
    let manifest = Manifest::new(map);
    assert!(check(&manifest, name.as_str()).is_err_and(|e| e.kind() == ErrorKind::NotFound));
}
