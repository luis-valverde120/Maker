use std::collections::HashMap;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct MakeConfig {
    pub frameworks: HashMap<String, FrameworkConfig>,
}

impl MakeConfig {
    pub fn get_framework(&self, name: &str) -> Option<&FrameworkConfig> {
        self.frameworks.get(name)
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct FrameworkConfig {
    pub language: String,
    pub init_cmd: Option<Vec<String>>,

    pub scripts_file: Option<String>,
    pub scripts_section: Option<String>,
    pub scripts: Option<HashMap<String, String>>,

    pub install_cmd: Vec<String>,
    pub install_dev_cmd: Vec<String>,
    pub dependencies: Vec<String>,
    pub dev_dependencies: Vec<String>,
    pub architectures: HashMap<String, ArchitectureConfig>,
}

impl FrameworkConfig {
    pub fn get_architecture(&self, name: &str) -> Option<&ArchitectureConfig> {
        self.architectures.get(name)
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct ArchitectureConfig {
    pub folders: Vec<String>,
    pub files: HashMap<String, String>,
}

