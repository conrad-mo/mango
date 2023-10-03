use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use std::fs::File;
use flate2::read::GzDecoder;
use tar::Archive;

#[derive(Debug, Deserialize, Serialize)]
pub struct Deps {
    #[serde(alias = "dependencies")]
    pub dependencies: HashMap<String, String>,
    #[serde(alias = "devDependencies")]
    pub dev_dependencies: HashMap<String, String>,
}

async fn decompress_tgz(name: String) -> Result<(), Box<dyn std::error::Error>> {
    let path = ("node_modules/{}.tgz", name);
    let tar_gz = File::open(path);
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive.unpack("node_modules")?;

    Ok(())
}