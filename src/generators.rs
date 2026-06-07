use std::path::PathBuf;
use std::path::Path;
use std::fs;
use std::process;
use std::collections::HashMap;
use serde_json::{Value, Map};
use toml_edit::{DocumentMut, Item, Table, value};
use indicatif::ProgressBar;

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
                spinner.abandon_with_message(format!("Process Canceled: \n{}", e));
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
    };

    for (file, path_template) in files {
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
            Err(_) => {
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

    let out = process::Command::new(&commands[0])
        .current_dir(path)
        .args(&commands[1..])
        .output()
        .map_err(|e| format!("Failed to execute command '{}': {}", commands.join(" "), e))?;

    if !out.status.success() {
        let err_msg = String::from_utf8_lossy(&out.stderr);
        println!("Error installing {}: {}", type_command, err_msg);
        println!("\nInstall command later using: {}", commands.join(" "));
    }

    Ok(())
}

/*
 * Add script in file if this is necesarry
 */
pub fn add_scripts(script: &HashMap<String, String>, field: &String, target_file: &String, path: &PathBuf, spinner: &ProgressBar) {

    let target = path.join(target_file);

    let extension = Path::new(target_file)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("");

    spinner.set_message(format!("Find file {}", target.display()));

    match extension {
        "json" => {
            inject_script_to_json(script, field, &target, spinner);
        },
        "toml" => {
            inject_script_to_toml(script, field, &target, spinner);
        },
        _ => {
            println!("Functionality for this type of file configure is not implemented yet");
        }
    };
}

fn inject_script_to_toml(script: &HashMap<String, String>, field: &str, target_file: &PathBuf, spinner: &ProgressBar) {

    let content = match fs::read_to_string(target_file) {
        Ok(val) => val,
        Err(e) => {
            spinner.abandon_with_message(format!("Error to read file {}", e));
            process::exit(1);
        },
    };

    let mut document = match content.parse::<DocumentMut>() {
        Ok(doc) => doc,
        Err(e) => {
            spinner.abandon_with_message(format!("Error to parse file {}", e));
            process::exit(1);
        },
    };


    let parts: Vec<&str> = field.split('.').collect();

    let mut current_table = document.as_table_mut();


    for part in &parts {
        if !current_table.contains_key(part) {
            current_table.insert(part, Item::Table(Table::new()));
        }

        current_table = current_table
            .get_mut(part)
            .unwrap()
            .as_table_mut()
            .unwrap_or_else(|| {
                spinner.abandon_with_message(format!("Error: '{}' is not a table in the TOML file", part));
                process::exit(1);
            });
    }

    for (key, val) in script {
        current_table.insert(key, value(val));
    }

    match fs::write(target_file, document.to_string()) {
        Ok(_) => {
            spinner.set_message(format!("Update complete file"));
        },
        Err(e) => {
            spinner.abandon_with_message(format!("Error for write file {}", e));
            process::exit(1);
        }
    };
}


fn inject_script_to_json(script: &HashMap<String, String>, field: &str, target_file: &PathBuf, spinner: &ProgressBar) {

    let mut json: Value = match serde_json::from_str(target_file.to_str().unwrap()) {
        Ok(val) => val,
        Err(_) => {
            println!("Error with read file");
            process::exit(1);
        },
    };

    if let Some(obj) = json.as_object_mut() {
        let script_block = obj.entry(field)
            .or_insert_with(|| Value::Object(Map::new()))
            .as_object_mut()
            .unwrap();

        for (key, val) in script {
            script_block.insert(key.clone(), Value::String(val.clone()));
        }
    }
    

    let json_update = match serde_json::to_string_pretty(&json) {
        Ok(json_file_update) => json_file_update,
        Err(e) => {
            spinner.abandon_with_message(format!("Error with update json {}", e));
            process::exit(1);
        },
    };

    match fs::write(target_file, json_update) {
        Ok(_) => {
            spinner.set_message(format!("Update complete file"));
        },
        Err(e) => {
            spinner.abandon_with_message(format!("Error for write file {}", e));
        },
    };

}
