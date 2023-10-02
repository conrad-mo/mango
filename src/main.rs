use clap::Parser;
use std::path::Path;
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Install node modules
    #[arg(short, long)]
    install: String,

    /// Add a node module to package.json
    #[arg(short, long)]
    add: String,
}

fn main() {
    let args = Args::parse();
    if !(Path::new("package.json").exists()){
        println!("Did not find package.json. Are you sure you are in project path?");
        return;
    }
    println!("Found package.json");
}