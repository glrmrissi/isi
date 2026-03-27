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
        #[arg(value_name = "PATH", num_args = 1..)]
        paths: Vec<String>,
    },
    Diff {
        #[arg(value_name = "OLD_HASH")]
        hash_old: Option<String>,
        #[arg(value_name = "NEW_HASH")]
        hash_new: Option<String>,
    },
    CatFile {
        #[arg(short = 'p')]
        hash: String,
    },
    Commit {
        #[arg(short = 'm', value_name = "MESSAGE")]
        message: String,
    },
    Push {
        /// Remote URL (overrides ISI_REMOTE env var, default: http://localhost:3000)
        #[arg(value_name = "REMOTE")]
        remote: Option<String>,
    },
}

