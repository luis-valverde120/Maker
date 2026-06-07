use crate::core::models::execution::{ ProjectExecution };
use crate::core::models::arguments::{ MakeConfig, FrameworkConfig };
use std::path::PathBuf;
use std::fs;
use std::process;

// file default config for frameworks and structure folders and architecture or commands use for a
// preconfigure a project
const CONFIGURE_YAML_BASE: &str = include_str!("../.././default_config.yaml"); 

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
 * Create a functionality for generate a default configure file this use 
 * This is for functionality Config --init
 * all configuration of default
 * */
pub fn create_configure_file() {
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

    if let Some(path) = path_config.parent() {
        let _ = fs::create_dir_all(path);
    }  

    match fs::write(&path_config, CONFIGURE_YAML_BASE) {
        Ok(_) => println!("Config file create sucessfully!"),
        Err(_) => {
            println!("Error: The configure file cannot create");
            process::exit(1);
        }
    }
}

/*
 * Read the file and get a structure with all configuration are in file config
 * This is use to get configuration and validate 
 * @Return MakeConfig
 * */
fn read_config_file(path: &str) -> Result<MakeConfig, serde_yaml::Error> {
    let config_file_content = match fs::read_to_string(path) {
        Ok(value) => value,
        Err(e) => {
            println!("Error: {}", e);
            process::exit(1);
        }
    };

    serde_yaml::from_str(&config_file_content)
}

/*
 * Handle configuration read the file configure to valide and create project 
 * 1. Validate if config file already exists
 * 2. Read config file 
 * 3. Save all structure of this file in The structures 
 * 4. Find configurations given for the User is describe in file
 * 5. 
 */
pub fn handle_config(project_name: &str, path: &PathBuf, framework: &str, architecture: &str, lang: &str) -> ProjectExecution {
    // valide config file already exists
    let path_config = match get_config_file() {
        Some(path) => path,
        None => {
            println!("Error: You need create a configure file\nUse command `maker config --init`");
            process::exit(0);
        }
    };
    
    let config = match read_config_file(&path_config.display().to_string()) {
        Ok(config) => config,
        Err(e) => {
            println!("{}", e);
            process::exit(1);
        } ,
    };

    let framework_conf: FrameworkConfig = match config.get_framework(framework) {
        Some(f) => f.clone(),
        None => {
            println!("Framework is not specified in config.yaml");
            process::exit(0);
        }
    };

    let architecture_conf = match framework_conf.get_architecture(&architecture) {
        Some(a) => a,
        None => {
            println!("Architecture is not specified in config.yaml");
            process::exit(0);
        }
    };

    ProjectExecution {
        name: project_name.to_string(),
        absolute_path: path.join(project_name),
        language: framework_conf.language.clone(),

        framework: framework.to_string(),

        // install dependencies
        init_cmd: framework_conf.init_cmd.clone(),
        install_cmd: framework_conf.install_cmd.clone(),
        install_dev_cmd: framework_conf.install_dev_cmd.clone(),
        dependencies: framework_conf.dependencies.clone(),
        dev_dependencies: framework_conf.dev_dependencies.clone(),

        //scripts
        scripts_file: framework_conf.scripts_file.clone(),
        scripts_section: framework_conf.scripts_section.clone(),
        scripts: framework_conf.scripts.clone(),

        // architecture
        architecture_name: architecture.to_string(), 
        architecture: architecture_conf.clone(),
    }
}
