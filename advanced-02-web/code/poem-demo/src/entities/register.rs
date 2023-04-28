use poem_openapi::{Object};
use serde::{Deserialize, Serialize};

#[derive(Object, Debug, Clone, Default, Serialize, Deserialize)]
pub struct Register {
    #[oai(read_only)]
    pub messge: String,
    pub code: i8,
    pub ok: bool
}
