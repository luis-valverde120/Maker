use std::path::PathBuf;
use std::collections::HashMap;
use crate::core::models::arguments::{ArchitectureConfig};

/*
 * This is the struct to the execution 
 * This contain all folders, structure files, commands, dependencies and dev_dependencies for the
 * project. This struct add a section scripts this is use for declare a scripts (e.g: npm run dev,
 * npm run build). For this is must a script_file and scripts_section if is declare a script_file
 * and we have scripts this secctions will be declare for default how scripts if the file for the
 * scripts is not declare this not add scripts, and if we have scripts this send a message indicate
 * we have a scripts not asigned becuase is need a script_file and optional scripts_section
 */
// configure file return for a execute
pub struct ProjectExecution {
    pub name: String,
    pub absolute_path: PathBuf,
    pub language: String,

    pub framework: String,
    
    // commands
    pub init_cmd: Option<Vec<String>>,
    
    // scripts
    pub scripts_file: Option<String>,
    pub scripts_section: Option<String>,
    pub scripts: Option<HashMap<String, String>>,

    pub install_cmd: Vec<String>,
    pub install_dev_cmd: Vec<String>,
    pub dependencies: Vec<String>,
    pub dev_dependencies: Vec<String>,

    // structure
    pub architecture_name: String,
    pub architecture: ArchitectureConfig,
}   
