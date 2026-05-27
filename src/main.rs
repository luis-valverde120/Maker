/*
 * This is a cli to handle structs and folder architecture a preconfigure all for proyect
 */
use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands, 
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
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
}

#[derive(Clone, Debug, ValueEnum)]
enum Framework  {
    Express,
    FastApi,
}

#[derive(Clone, Debug, ValueEnum)]
enum Architecture {
    Mvc,
    Hexagonal,
    Ddd,
}

// this is just for use a express framework
#[derive(Clone, Debug, ValueEnum)]
enum Language { 
    Ts,
    Js,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::New { name, path, framework, arch, lang } => {
            println!("Creating new project: {}", name);

            if let Some(fw) = framework {
                println!("Selected framework: {:?}", fw);
            }

            if let Some(a) = arch {
                println!("Selected architecture: {:?}", a);
            }

            if let Some(l) = lang {
                println!("selected language: {:?}", l);
            }
        }
    }
}
