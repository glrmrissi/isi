use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "isi")]
#[command(about = "A minimalist recreation of Git in Rust")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Init,
    Add {
        #[arg(value_name = "FILE")]
        path: String,
    },
    Diff {
        hash_old: String,
        hash_new: String,
    },
    CatFile {
        #[arg(short = 'p')]
        hash: String,
    },
}
