use clap::{Parser, Subcommand, ValueEnum};

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

        #[clap(short, long, value_enum)]
        framework: Option<Framework>,

        #[clap(short, long, value_enum)]
        arch: Option<Architecture>,

        #[clap(long, value_enum)]
        lang: Option<Language>,
    },
    Config {
        #[clap(long)]
        init: bool
    }
}

#[derive(Clone, Debug, ValueEnum)]
pub enum Framework  {
    Express,
    FastApi,
}

#[derive(Clone, Debug, ValueEnum)]
pub enum Architecture {
    Mvc,
    Hexagonal,
    Ddd,
}

// this is just for use a express framework
#[derive(Clone, Debug, ValueEnum)]
pub enum Language { 
    Ts,
    Js,
}

