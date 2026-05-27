use std::path::Path;
use std::fs;
use std::io;

mod generators;

fn is_valid_path(path_project: &str) -> bool {
    Path::new(path_project).exists()
}

pub fn create_folders_structure(path_project: Option<&str>) {
    if let Some(p) = path_project {
        println!("We have path!");
    } else {
        println!("We donnot have path given for user");
    } 
}

pub fn create_project(name_project: &str) {
    if !is_valid_path(name_project) {
        println!("Path no valid");
        return;
    }

    fs::
}
