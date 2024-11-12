mod types;

use mango::{deps_download, lock_gen };
use mango::types::Deps;
use clap::{Parser, Subcommand};
use std::path::Path;
use tokio::io::AsyncReadExt;
use std::fs;

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
    Add { packagename: String },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Install {} => {
            if !(Path::new("package.json").exists()) {
                println!("Did not find package.json. Are you sure you are in project path?");
                println!("Exiting");
                return;
            }
            println!("Found package.json");
            if !(Path::new("mango.lock").exists()) {
                println!("Did not find mango.lock. Generating mango.lock");
                lock_gen().await;
            }
            let mut deps = tokio::fs::File::open("package.json")
                .await
                .expect("Failed to open package.json");
            let mut contents = String::new();
            deps.read_to_string(&mut contents)
                .await
                .expect("Failed to read package.json");
            let mut parsed_data: Deps =
                serde_json::from_str(&contents).expect("Failed to parse JSON");
            fs::create_dir_all("node_modules").expect("Failed to create node_modules folder");
            deps_download(&mut parsed_data.dependencies).await;
            deps_download(&mut parsed_data.dev_dependencies).await;
        }
        Commands::Add { packagename } => {
            if !(Path::new("package.json").exists()) {
                println!("Did not find package.json. Are you sure you are in project path?");
                println!("Exiting");
                return;
            }
            println!("Adding {} to project", packagename);
        }
    }
}
