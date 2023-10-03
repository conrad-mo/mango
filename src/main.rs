mod types;

use std::collections::HashMap;
use clap::{Parser, Subcommand};
use std::path::Path;
// use crate::types::{decompress_tgz, Deps};
use crate::types::{Deps};
use std::fs;
use reqwest::{Client, Error};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}
#[derive(Subcommand, Debug)]
enum Commands {
    /// Install node_modules
    Install {},
    /// Add a node dependency to project
    Add {packagename: String}
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    match &cli.command {
       Commands::Install {} => {
           if !(Path::new("package.json").exists()){
               println!("Did not find package.json. Are you sure you are in project path?");
               println!("Exiting");
               return;
           }
           println!("Found package.json");
           let mut deps = tokio::fs::File::open("package.json").await.expect("Failed to open package.json");
           let mut contents = String::new();
           deps.read_to_string(&mut contents).await.expect("Failed to read package.json");
           let mut parsed_data: Deps = serde_json::from_str(&contents).expect("Failed to parse JSON");
           fs::create_dir_all("node_modules").expect("Failed to create node_modules folder");
           deps_download(&mut parsed_data.dependencies).await;
           deps_download(&mut parsed_data.dev_dependencies).await;
       }
        Commands::Add { packagename } =>{
            if !(Path::new("package.json").exists()){
                println!("Did not find package.json. Are you sure you are in project path?");
                println!("Exiting");
                return;
            }
            println!("Adding {} to project", packagename);
        }
    }
}

async fn deps_search (package_name: String){
    let mut deps = tokio::fs::File::open(format!("node_modules/{}/package.json", package_name)).await.expect("Failed to open package.json");
    let mut contents = String::new();
    deps.read_to_string(&mut contents).await.expect("Failed to read package.json");
    let mut parsed_data: Deps = serde_json::from_str(&contents).expect("Failed to parse JSON");
    deps_download(&mut parsed_data.dependencies).await;
    deps_download(&mut parsed_data.dev_dependencies).await;
}

async fn deps_download(depshash: &mut HashMap<String, String>){
    for (key, value) in &mut *depshash{
        let mut version: &str = value;
        let mut name: &str = key;
        let mut url: String = String::from("");
        if value.contains("^"){
            version = &value[1..];
        }
        if key.contains("@"){
            name = &key[key.find("/").unwrap() + 1..];
            url = format!("https://registry.npmjs.org/{}/-/{}-{}.tgz", key, name, version);
        }
        else{
            url = format!("https://registry.npmjs.org/{}/-/{}-{}.tgz", key, key, version);
        }
        let url_pointer: &str = &url;
        let _ = download_module(url_pointer, name).await;
    }
    depshash.clear();
}

async fn download_module(url: &str, name: &str) -> Result<(), Error> {
    println!("{}", name);
    println!("{}", url);
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
        eprintln!("Failed to download {}: Status code: {:?}", name, response.status());
    }
    println!("Done downloading {}", name);
    //decompress_tgz(String::from(name));
    Ok(())
}
