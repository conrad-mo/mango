mod types;

use std::fs::File;
use std::io::Read;
use clap::{Parser, Subcommand};
use std::path::Path;
use crate::types::Deps;

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

fn main() {
    let cli = Cli::parse();
    match &cli.command {
       Commands::Install {} => {
           if !(Path::new("package.json").exists()){
               println!("Did not find package.json. Are you sure you are in project path?");
               println!("Exiting");
               return;
           }
           println!("Found package.json");
           let deps = File::open("package.json").expect("Failed to open package.json");
           let mut contents = String::new();
           deps.take(1024).read_to_string(&mut contents).expect("Failed to read package.json");
           let parsed_data: Deps = serde_json::from_str(&contents).expect("Failed to parse JSON");

           println!("Dependencies: {:?}", parsed_data.dependencies);
           println!("Dev Dependencies: {:?}", parsed_data.dev_dependencies);
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