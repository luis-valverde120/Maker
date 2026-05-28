/*
 * This is a cli to handle structs and folder architecture a preconfigure all for proyect
 */
use clap::{Parser, Subcommand, ValueEnum};
use dialoguer::Confirm;
use std::env;

mod generators;

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

            match path {
                None => {

                    let mut value_of_current_path: String = String::from("");

                    if let Ok(current_path) = env::current_dir() {
                        value_of_current_path = current_path.display().to_string();
                    };

                    let mut string_folder_project = String::from("You create the project in ");

                    string_folder_project.push_str(&value_of_current_path);
                    string_folder_project.push_str("\nContinue?");

                    let confirmation_path = Confirm::new()
                        .with_prompt(string_folder_project)
                        .default(true)
                        .interact();

                    match confirmation_path {
                        Err(e) => println!("Error {}", e),
                        Ok(confirmation) => {
                            println!("{}", confirmation);
                        },
                    };
                },
                Some(path) => println!("path: {}", path),
            }
            
            generators::create_project(name);
            
        }
    }
}
