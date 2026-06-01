use std::path::PathBuf;
use crate::core::models::arguments::{ArchitectureConfig};

// configure file return for a execute
pub struct ProjectExecution {
    pub name: String,
    pub absolute_path: PathBuf,
    pub language: String,

    pub framework: String,
    
    // commands
    pub init_cmd: Option<Vec<String>>,
    pub script: Option<Vec<String>>,
    pub install_cmd: Vec<String>,
    pub install_dev_cmd: Vec<String>,
    pub dependencies: Vec<String>,
    pub dev_dependencies: Vec<String>,

    // structure
    pub architecture: ArchitectureConfig,
}   
