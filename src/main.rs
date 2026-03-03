use std::process;

use clap::Parser;
use isi::*;
use isi::cli::Cli;

fn main() {
    let cli = Cli::parse(); 

    let result = match cli.command {
        Commands::Init => isi::commands::init::execute(),
        Commands::Add { path } => isi::commands::add::execute(&path),
        Commands::Diff { hash_old, hash_new } => isi::commands::diff::execute(&hash_old, &hash_new),
        Commands::CatFile { hash } => isi::commands::cat::execute(&hash)
    };

    if let Err(e) = result {
        eprintln!("Error in execution: {}", e);
        process::exit(1);
    }
}