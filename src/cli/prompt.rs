use crate::core::models::execution::{ProjectExecution};
use dialoguer::Confirm;
use std::process;

pub fn validate_configure_project(configure: &ProjectExecution) -> bool {
    println!("Resume Configuration:");
    println!("- Project Name: {}", configure.name);
    println!("- Path: {}", configure.absolute_path);
    println!("- Framework: {}", configure.framework.to_string());
    println!("- Architecture: {}", configure.architecture.to_string());

    Confirm::new()
        .with_prompt("Do you want continue?")
        .default(true)
        .interact()
        .unwrap()
}
