use std::collections::HashMap;

lazy_static! {
    pub static ref MAPPING: HashMap<&'static str, &'static str> =
        [("zip", "unzip"), ("tar.gz", "tar -xvzf")]
            .iter()
            .copied()
            .collect();
}
