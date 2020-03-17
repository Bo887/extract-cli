use std::collections::HashMap;

lazy_static! {
    pub static ref MAPPING: HashMap<&'static str, &'static str> = [
        ("zip", "unzip"),
        ("tar.gz", "tar -xvzf"),
        ("tar.xz", "tar -xvJf"),
        ("tar.bz2", "tar -xvjf")
    ]
    .iter()
    .copied()
    .collect();
}
