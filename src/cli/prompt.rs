use crate::core::models::execution::{ProjectExecution};
use dialoguer::Confirm;

pub fn validate_configure_project(configure: &ProjectExecution) -> bool {
    println!("Resume Configuration:");
    println!("- Project Name: {}", configure.name);
    println!("- Path: {}", configure.absolute_path.display().to_string());
    println!("- Framework: {}", configure.framework.to_string());
    println!("- Architecture: {}", configure.architecture_name);

    Confirm::new()
        .with_prompt("Do you want continue?")
        .default(true)
        .interact()
        .unwrap()
}
