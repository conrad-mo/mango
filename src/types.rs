use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Deps {
    #[serde(alias = "dependencies")]
    pub dependencies: HashMap<String, String>,
    #[serde(alias = "devDependencies")]
    pub dev_dependencies: HashMap<String, String>,
}