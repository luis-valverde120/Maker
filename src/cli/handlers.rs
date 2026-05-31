use crate::cli::args::{Cli, Commands};
use crate::cli::handler_config;
use dialoguer::{Confirm, Input};
use std::process;
use std::path::{Path, PathBuf};
use std::env;
use std::fs;

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

/**
 * Especifie a path to use
 */
fn set_path() -> String {
    let new_path: String = Input::new()
        .with_prompt("Specify the path: ")
        .validate_with(|input: &String| -> Result<(), &str> {
            if input.trim().is_empty() {
                return Err("Path is void");
            }

            if is_valid_path(input) {
                Ok(())
            } else {
                Err("Path is invalid")
            }
        })
        .interact()
        .unwrap();

    return new_path;
}

/**
 * This is a funciton used to create a path with the name given for the user
 */
fn handler_path(path: Option<&str>, name: &str) -> Result<PathBuf, String> {

    // validate if exist a path if not exist return current path
    let selected_path = match path {
        Some(path) => path,
        None => {
            &match env::current_dir() {
                Ok(path) => {
                    let confirmation = Confirm::new()
                        .with_prompt(format!("The project is create in {}\nDo you want continue?", path.display().to_string()))
                        .default(true)
                        .interact()
                        .unwrap();

                    if confirmation {
                        path.display().to_string()
                    } else {
                        set_path() // return a path
                    }
                },
                Err(e) => {
                    println!("We have a error {}", e);
                    process::exit(1);
                }
            }
        },
    };

    let new_path = Path::new(selected_path).join(name);

    match fs::create_dir(&new_path) {
        Ok(_) => {
            println!("Create a new directory sucessfully");
            Ok(new_path)
        },
        Err(e) => {
            Err(format!("An Error has ocurrent {}", e))
        }
    }
}

/**
 * This function is used for handler the framework used for the project this create and install all
 * dependencies for the project works.
 */
fn handler_framework(framework: &str) -> Result<String, String> {
    if framework.trim().is_empty() {
        return Err(format!("Errro"));
    }
    Ok(format!("ok"))
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

            let path_project: PathBuf = match handler_path(path.as_deref(), &name) {
                Ok(path) => path,
                Err(e) => {
                    println!("Error {}", e);    
                    process::exit(1);
                }
            };

            /*
            let mut project = match handler_config::handle_config(
                framework_selected, 
                architecture_selected, 
                language_selected
            ) {
                Some(p) => p,
                None => ProjectExecution{}, 
            };
            */

            if let Some(_) = handler_config::handle_config(
                name,
                path_project,
                framework_selected,
                architecture_selected,
                language_selected
            ) {
                println!("Ok");
            }

            println!("Creating a new project {} ...", name);

            
            
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
