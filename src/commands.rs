use std::collections::HashMap;
use crate::utils;

fn generate_command_map() -> HashMap<String, String> {
    let mut command_mapping = HashMap::new();

    command_mapping.insert("zip".to_string(), "unzip".to_string());
    command_mapping.insert("tar.gz".to_string(), "tar -xvzf".to_string());

    command_mapping
}

pub fn generate_command(path: String) -> Option<String> {
    let command_mapping = generate_command_map();
    let extension = utils::get_extension(&path);

    if !extension.is_some() {
        return None
    }
    else if !command_mapping.contains_key(extension.unwrap()) {
        return None
    }
    Some(command_mapping[extension.unwrap()].clone())
}
