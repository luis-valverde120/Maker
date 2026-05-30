use crate::cli::args::{Cli, Commands};
use dialoguer::{Confirm, Input};
use std::process;
use std::path::{Path, PathBuf);
use std::env;
use std::fs;

// file default config for frameworks and structure folders and architecture or commands use for a
// preconfigure a project
const CONFIGURE_YAML_BASE: &str = include_str!("../core/default_config.yaml"); 

/*
 * this use to valid if the config 
 */
fn get_config_file() -> Option<PathBuf>{
    match dirs::config_dir() {
        Some(path) => Some(path.join("maker").join("config.yaml")),
        None => None,
    }           
}

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

/*
 * Create a functionality for generate a default configure file this use 
 * This is for functionality Config --init
 * all configuration of default
 * @path configure folder
 * */
fn create_configure_file(path: &str) {
    let path_config = match get_config_file() {
        Some(path) => path,
        None => {
             println!("Error: not find configure folder");
             return;
        }
    };

    // this is already exists
    if path_config.exists() {
        println!("The configure is already exist");
        return;
    }

    if let Some(path) = path_config.current() {
        let _ = fs::create_dir_all(path);
    }  

    match fs::write(&path_config, CONFIGURE_YAML_BASE) {
        Ok(_) => println!("Config file create sucessfully!"),
        Err(e) => {
            println!("Error: The configure file cannot create");
            process::exit(1);
        }
    }
}

/*
 * Add functionality of handler a configure file this will be create automatically 
 * for make a this app more usefull for diferent configurations
 * @path: configure folder 
 */
fn handler_config_file() {

}

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
fn handler_path(path: Option<&str>, name: &str) -> Result<String, String> {

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
            Ok(format!("Create a new directory sucessfully"))
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
    return Ok(format!("ok"));
}

/*
 * This handler the command the flags that use and determine what flags is missing
 */
pub fn handler_command(cli: Cli) -> Result<(), String> {

    match &cli.command {
        Commands::New { name, path, framework, arch, lang } => {

            if !is_valid_name(&name) {
                println!("Error with name {}, this is not valid", name);
                process::exit(1);
            }

            println!("Create a new project {}", name);

            match handler_path(path.as_deref(), &name) {
                Ok(result) => {
                    println!("{}", result);
                },
                Err(e) => {
                    println!("Error {}", e);    
                    process::exit(1);
                }
            }

            match handler_framework(framework) {
                Ok(result) => {
                    println!("{}", result); 
                }
            }
       },
       Commands::Config { init } => {
           if !init => {
               println!("You need add the flag --init");
               process::exit(0);
           }
            
           handler_config_file();

       }
   } 
   Ok(())
}
