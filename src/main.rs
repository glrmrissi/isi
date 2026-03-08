use std::process;

use clap::Parser;
use isi::*;
use isi::cli::Cli;

fn main() {
    let cli = Cli::parse(); 

    let result = match cli.command {
        Commands::Init => isi::commands::init::execute(),
        Commands::Add { paths } => isi::commands::add::execute(&paths),
        Commands::Diff { hash_old, hash_new } => isi::commands::diff::execute(hash_old.as_deref(), hash_new.as_deref()),
        Commands::CatFile { hash } => isi::commands::cat::execute(&hash)
    };

    if let Err(e) = result {
        eprintln!("Error in execution: {}", e);
        process::exit(1);
    }
}