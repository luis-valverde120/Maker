use crate::cli::args::{Cli, Commands};
use crate::core::models::execution::{ProjectExecution};
use crate::core::models::arguments::{ArchitectureConfig};
use crate::cli::handler_config;
use crate::cli::prompt;
use dialoguer::{Confirm, Input};
use indicatif::{ProgressBar, ProgressStyle}
use std::process;
use std::path::{Path, PathBuf};
use std::env;
use std::fs;
use std::time::Duration;

/*
 * This function valid if a name has spaces or special characters
 * Just valid caracter is _ 
 */
fn is_valid_name(name: &str) -> bool {
    if name.trim().is_empty() {
        return false;
    } 

    name.chars().all(|c| c.is_alphanumeric() || c == '_' || c == ' ')
}

/*
 * Validate a path is this exist or is valid
 * Validate the path is not empty
 * First need validate if this path exits
 */
fn is_valid_path(path_project: &str) -> bool {
    if path_project.trim().is_empty() {
        return false;
    }

    Path::new(path_project).exists()
}

fn use_current_dir() -> PathBuf {
    match env::current_dir() {
        Ok(path) => {
            let confirmation = Confirm::new()
                .with_prompt(format!("The project is create in {}\nDo you want continue?", path.display().to_string()))
                .default(true)
                .interact()
                .unwrap();

            if confirmation {
                path
            } else {
                println!("Canceled");
                process::exit(0);
            }
        },
        Err(e) => {
            println!("Error with current path \n{}", e);
            process::exit(1);
        }
    }
}

/**
 * This is a funciton used to create a path with the name given for the user
 */
fn handler_path(path: PathBuf) -> Result<(), String> {
    match fs::create_dir(&path) {
        Ok(_) => {
            println!("Create a new directory sucessfully");
            Ok(())
        },
        Err(e) => {
            Err(format!("An Error has ocurrent {}", e))
        }
    }
}

/**
 * Create a structure of folder inside of path of project
 */
fn create_architecture_folder(folders: &Vec<String>, path: &PathBuf, spinner: &ProgressBar) {
    for folder in folders {
        let new_folder = path.join(folder);
        match fs::create_dir_all(new_folder) {
            Ok(_) => {
                spinner.set_message(format!("Creating: {}", folder));
            },
            Err(e) => {
                spinner.abandon_with_message(format!("Process Canceled: \n{}", e))
                process::exit(1);
            }
        };
    }
}

fn create_architecture_files(files: &Vec<String>, path: &PathBuf, spinner: &ProgressBar) {
    for file in files {
        // i need a path from files no a content
    }
}

/*
 * This handler the command the flags that use and determine what flags is missing
 * *New Configuration 
 * 1. validate a name and create the folder where we work
 * 2. Validate the config file is already exist 
 *      In this case need Read the configure file 
 * 3. Validate if the framework already exists describe in configure file
 * 4. Get language for this configuration
 * 5. Init project using the framework 
 * 6. Install all dependencies and dev_dependencies 
 * 7. Generate a structure of folder
 *
 *
 * Especial cases
 * Javascript
 * this need to aclare the language used this case use Javascript or Typescript
 */
pub fn handler_command(cli: Cli) -> Result<(), String> {

    match &cli.command {
        Commands::New { name, path, framework, arch, lang } => {

            let framework_selected = match framework {
                Some(f) => f,
                None => &String::from(""),
            };

            let architecture_selected = match arch {
                Some(a) => a,
                None => &String::from(""),
            };

            let language_selected = match lang {
                Some(l) => l,
                None => &String::from(""),
            };

            if !is_valid_name(&name) {
                println!("Error with name {}, this is not valid", name);
                process::exit(0);
            }

            let path_selected = match path {
                Some(p) => {
                    if is_valid_path(p) {
                        Path::new(p)
                    } else {
                        println!("Error with path");
                        process::exit(0);
                    }
                },
                None => use_current_dir(),
            };

            let spinner = ProgressBar::new_spinner();

            spinner.set_style(
                ProgressStyle::default_spinner().template("{spinner.green} {msg}").unwrap(),
            );

            spinner.enable_steady_tick(Duration::from_millis(100));


            spinner.set_message(format!("Charge the configuration from config.yaml"));

            // Chare configure
            let configure: ProjectExecution  = handler_config::handle_config(
                name,
                path_project,
                framework_selected,
                architecture_selected,
                language_selected
            );
            
            if !prompt::validate_configure_project(&configure) {
                println!("Canceling...");
                process::exit(0);
            };

            spinner.set_message(format!("Create a folder project {}", name))

            // create a project
            match handler_path(&path_selected.join(name)) {
                Ok() => println!("Succesfully create folder");,
                Err(e) => {
                    spinner.abandon_with_message("Process Canceled: {}", e)
                    process::exit(1);
                }
            };

            spinner.set_message(format!("Create structure folder from architecture {}", architecture_selected))

            // create structure from architectur
            create_architecture_folder(&configure.architecture.folders, &configure.absolute_path, &spinner);
            
            spinner.set_message(format!("Create files for project"))
            // create structure of files
            create_architecture_files(&configure.architecture.files, &configure.absolute_path, &spinner);

       },
       Commands::Config { init } => {
           if !init {
               println!("You need add the flag --init");
               process::exit(0);
           }
            
           handler_config::create_configure_file();
       }
   } 
   Ok(())
}
