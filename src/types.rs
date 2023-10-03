use std::collections::HashMap;
use std::fs;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use flate2::read::{GzDecoder};
use std::io::prelude::*;
use std::path::PathBuf;
use tar::Archive;

#[derive(Debug, Deserialize, Serialize)]
pub struct Deps {
    #[serde(alias = "dependencies")]
    pub dependencies: HashMap<String, String>,
    #[serde(alias = "devDependencies")]
    pub dev_dependencies: HashMap<String, String>,
}

pub(crate) async fn decompress_tgz(name: String) {
    println!("Unzipping {}", name);
    let tar = File::open(format!("node_modules/{}.tgz", name)).unwrap();
    let dec = GzDecoder::new(tar);
    let mut a = Archive::new(dec);
    for entry in a.entries().unwrap() {
        let mut entry = entry.unwrap();
        let mut entry_path = PathBuf::new();
        entry_path.push("node_modules/");
        entry_path.push(&name);
        match entry.path().unwrap().strip_prefix("package") {
            Ok(subpath) => {
                entry_path.push(subpath);
                if let Some(parent) = entry_path.parent() {
                    std::fs::create_dir_all(parent).unwrap();
                }
                entry.unpack(&entry_path).unwrap();
            }
            Err(error) => {
                println!("Failed to strip prefix: {:?}", error);


            }
        }
    }
    let _ = fs::remove_file(format!("node_modules/{}.tgz", name));
    println!("Done unzipping");
}