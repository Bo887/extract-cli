use std::collections::HashMap;
use crate::utils;

fn generate_command_map() -> HashMap<String, String> {
    let mut command_mapping = HashMap::new();

    command_mapping.insert(".zip".to_string(), "unzip".to_string());
    command_mapping.insert(".tar.gz".to_string(), "tar -xvzf".to_string());

    command_mapping
}

pub fn generate_command(path: String) -> Option<Vec<String>> {
    let command_mapping = generate_command_map();

    None
}
