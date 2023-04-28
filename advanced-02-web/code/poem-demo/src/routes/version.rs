use poem_openapi::{
    payload::Json,
    ApiResponse, OpenApi,
};
use crate::{
    entities::version::{ Version },
};

use super::ApiTags;
pub struct VersionApi;

#[OpenApi]
impl VersionApi {
    #[oai(path = "/api/version", method = "get", tag = "ApiTags::VersionApi")]
    async fn get_version(&self) -> GetVersionsResponse {
        let version = Version {
            name: String::from("Chaineye"),
            description: String::from("链眼社区"),
            version: String::from("V0.1.0"),
        };
        GetVersionsResponse::Ok(Json(version))
    }
}

#[derive(ApiResponse)]
pub enum GetVersionsResponse {
    #[oai(status = 200)]
    Ok(Json<Version>),

    #[oai(status = 500)]
    Internal,
}


