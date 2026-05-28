use crate::cli::args::{Cli, Commands};
use std::process;
use std::path::Path;
use std::env;

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

    Path::new(path_project).exist()
}

fn handler_path() {

}

/*
 * This handler the command the flags that use and determine what flags is missing
 */
pub fn handler_command(cli: Cli) -> Result<(), String> {
    let mut current_path = String::from(""); 

    match &cli.command {
        Commands::New { name, path, framework, arch, lang } => {

            if !is_valid_name(&name) {
                println!("Error with name {}, this is not valid", name);
                process::exit(1);
            }

            println!("Create a new project {}", name);


            match path {
                Some(path) => {
                    is_valid_path(&path);

                },
                None => {
                    let current_path = env::current_dir();
                    if let Err(e) = current_path {
                        println!("We have a error using the current dir \n {}", env::current_dir().display());
                        process::exit(1);
                    }        
                    

                },
            }
       } 
   } 
   Ok(())
}
