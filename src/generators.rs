use std::path::Path;
use std::fs;
use std::process;
use std::collections::HashMap;

/**
 * Create a structure of folder inside of path of project
 */

pub fn create_architecture_folder(folders: &Vec<String>, path: &PathBuf, spinner: &ProgressBar) {
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

pub fn create_architecture_files(files: &HashMap<String, String>, path: &PathBuf, spinner: &ProgressBar) {
    // get configurations file
    let templates_dir: PathBuf = match dirs::config_dir() {
        Some(path) => path.join("maker"),
        None => {
            spinner.abandon_with_message(format!("Path of templates not found"));
            process::exit(1);
        }
    }

    for {file, path_template} in files {
        // file
        let target_file = path.join(file);

        spinner.set_message(format!("Copy template {}", path_template));
        
        // get content
        match fs::read_to_string(templates_dir.join(path_template)) {
            Ok(content) => {
                if let Err(e) = fs::write(&target_file, content) {
                    spinner.abandon_with_message(format!("Error with writing template {}, {}", file, e));
                    process::exit(1);
                }
            },
            Err(e) => {
                spinner.abandon_with_message(format!("Error template not found {}", path_template));
                process::exit(1);
            }
        };   
    }

}

/*
 * This is use for install dependencies and dev_dependencies in project
 * type_command is the string of filed dependencies or dev_dependencies
 * this is used just output string to define what is install
 * dependencies and dev_dependencies are Vec<String> that because is use this function to install
 * all without declare a another funcion that execute the seme logic
 */
pub fn install_commands(commands: &Vec<String>, type_command: &str, path: &PathBuf) -> Result<(), String> {
    if commands.is_empty() {
        return Ok(());
    }

    match process::Command::new(&commands[0])
        .current_dir(path)
        .args(&commands[1..])
        .output() {
            Ok(output) => {
                if !output.status() {
                    let err_text = String::from_utf8_lossy(&output.stderr);
                    println!("Error with install {}\n {}", type_command, err_text);
                    println!("\nInstall {} later using: \n{}", type_command, commands.join(" "));
                }
                Ok(())
            }
            Err(e) => {
                format!("Error: {}", e)
            }
        }
}
