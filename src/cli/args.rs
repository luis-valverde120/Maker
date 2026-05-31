use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    New {
        name: String,

        #[clap(long)]
        path: Option<String>,

        #[clap(short, long)]
        framework: Option<String>,

        #[clap(short, long)]
        arch: Option<String>,

        #[clap(long)]
        lang: Option<String>,
    },
    Config {
        #[clap(long)]
        init: bool
    }
}

