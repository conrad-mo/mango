pub mod types;

use crate::types::Deps;
use flate2::read::GzDecoder;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::path::PathBuf;
use tar::Archive;
use reqwest::{Client, Error};
use std::path::Path;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub async fn deps_search(package_name: String) {
    let mut deps = tokio::fs::File::open(format!("node_modules/{}/package.json", package_name))
        .await
        .expect("Failed to open package.json");
    let mut contents = String::new();
    deps.read_to_string(&mut contents)
        .await
        .expect("Failed to read package.json");
    let mut parsed_data: Deps = serde_json::from_str(&contents).expect("Failed to parse JSON");
    deps_download(&mut parsed_data.dependencies).await;
    deps_download(&mut parsed_data.dev_dependencies).await;
}

pub async fn deps_download(depshash: &mut HashMap<String, String>) {
    for (key, value) in &mut *depshash {
        let mut version: &str = value;
        let mut name: &str = key;
        let mut url: String = String::from("");
        if value.contains("^") {
            version = &value[1..];
        }
        if key.contains("@") {
            name = &key[key.find("/").unwrap() + 1..];
            url = format!(
                "https://registry.npmjs.org/{}/-/{}-{}.tgz",
                key, name, version
            );
        } else {
            url = format!(
                "https://registry.npmjs.org/{}/-/{}-{}.tgz",
                key, key, version
            );
        }
        let url_pointer: &str = &url;
        let _ = download_module(url_pointer, name).await;
    }
    depshash.clear();
}

pub async fn download_module(url: &str, name: &str) -> Result<(), Error> {
    println!("{}", name);
    println!("{}", url);
    if !Path::new(&format!("node_modules/{}", name)).exists() {
        let client = Client::new();
        let response = client.get(url).send().await?;

        if response.status().is_success() {
            let path = format!("node_modules/{}.tgz", name);
            let mut file = tokio::fs::File::create(&path)
                .await
                .expect("Failed to create or open file");
            let content = response.bytes().await?;
            file.write_all(&content)
                .await
                .expect("Failed to write content to file");

            println!("File downloaded successfully to: {:?}", path);
        } else {
            eprintln!(
                "Failed to download {}: Status code: {:?}",
                name,
                response.status()
            );
        }
        println!("Done downloading {}", name);
        decompress_tgz(String::from(name)).await;
    }
    Ok(())
}

pub async fn lock_gen() {}
pub(crate) async fn decompress_tgz(name: String) {
    println!("Unzipping {}", name);
    let tar = File::open(format!("node_modules/{}.tgz", name)).unwrap();
    if !std::path::Path::new(&format!("node_modules/{}.tgz", name)).exists() {
        println!("File not found: {:?}", format!("node_modules/{}.tgz", name));
        return;
    }
    let dec = GzDecoder::new(tar);
    let mut a = Archive::new(dec);
    for entry in a.entries().unwrap() {
        match entry {
            Ok(mut value) => {
                let mut entry_path = PathBuf::new();
                entry_path.push("node_modules/");
                entry_path.push(&name);
                match value.path().unwrap().strip_prefix("package") {
                    Ok(subpath) => {
                        entry_path.push(subpath);
                        if let Some(parent) = entry_path.parent() {
                            std::fs::create_dir_all(parent).unwrap();
                        }
                        value.unpack(&entry_path).unwrap();
                    }
                    Err(error) => {
                        println!("No package folder {:?}", error);
                        match value.path().unwrap().strip_prefix(&name) {
                            Ok(subpath) => {
                                entry_path.push(subpath);
                                if let Some(parent) = entry_path.parent() {
                                    std::fs::create_dir_all(parent).unwrap();
                                }
                                value.unpack(&entry_path).unwrap();
                            }
                            Err(error) => {
                                println!("No subfolder, defaulting to normal {:?}", error);
                                let tar = File::open(format!("node_modules/{}.tgz", name)).unwrap();
                                let dec = GzDecoder::new(tar);
                                let mut a = Archive::new(dec);
                                a.unpack(format!("node_modules/{}", name)).unwrap();
                            }
                        }
                    }
                }
            }
            Err(error) => {
                println!("Failed to unwrap entry {:?}", error)
            }
        }
    }
    let _ = fs::remove_file(format!("node_modules/{}.tgz", name));
    println!("Done unzipping");
}
