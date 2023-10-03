use clap::{Parser, Subcommand};
use std::path::Path;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}
#[derive(Subcommand, Debug)]
enum Commands {
    Install {},
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
        }
        Commands::Add { packagename } =>{
            println!("Adding {} to project", packagename);
        }
    }

}