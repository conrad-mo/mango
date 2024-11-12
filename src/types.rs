use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct Deps {
    #[serde(alias = "dependencies")]
    pub dependencies: HashMap<String, String>,
    #[serde(alias = "devDependencies")]
    pub dev_dependencies: HashMap<String, String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DepLeaf {
    pub name: String,
    pub deps: Vec<String>,
}
