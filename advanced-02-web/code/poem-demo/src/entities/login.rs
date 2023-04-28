use poem_openapi::{Object};
use serde::{Deserialize, Serialize};

#[derive(Object, Debug, Clone, Default, Serialize, Deserialize)]
pub struct Register {
    #[oai(read_only)]
    pub ok: bool,
    pub code: i8,
    pub message: String,
    pub token: String,
    pub user_name: String,
    pub user_introduce: String,
}
