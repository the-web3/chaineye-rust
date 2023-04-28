use poem::{ Route};
use poem_openapi::{OpenApiService, Tags};

pub mod version;
pub mod login;
pub mod register;

#[derive(Tags)]
enum ApiTags {
    VersionApi,
    Login,
    Register,
}

pub fn routes() -> Route {
    let openapi_service = OpenApiService::new(
        (version::VersionApi),
        // (login::Login),
        // (register::Register),
        "Let's Science API",
        "0.1",
    ).server("http://localhost:3000/api");
     Route::new()
         .nest_no_strip("/api", openapi_service)
}