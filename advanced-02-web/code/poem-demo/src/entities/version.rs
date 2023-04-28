use poem_openapi::{Object};
use serde::{Deserialize, Serialize};

#[derive(Object, Debug, Clone, Default, Serialize, Deserialize)]
pub struct Version {
    #[oai(read_only)]
    pub name: String,
    pub description: String,
    pub version: String
}
