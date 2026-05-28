/*
 * This is a cli to handle structs and folder architecture a preconfigure all for proyect
 */
use clap::{Parser, Subcommand, ValueEnum};
use dialoguer::Confirm;
use std::env;
use maker::cli::handlers;
use maker::cli::args::Cli;


fn main() {
    let cli = Cli::parse();

    handlers::handler_command(cli);

    /*
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
    */
}
