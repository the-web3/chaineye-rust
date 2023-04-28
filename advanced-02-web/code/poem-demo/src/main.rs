use poem::{
    endpoint::PrometheusExporter,
    listener::TcpListener,
    middleware::Cors,
    EndpointExt, Server,
};
use tracing_subscriber::{layer::SubscriberExt};

pub mod core;
pub mod entities;
pub mod routes;

#[tokio::main]
async fn main() {
    let port = match std::env::var("PORT") {
        Ok(port) => port.parse().expect("PORT is not a valid u32"),
        Err(_) => 3000,
    };

    let cors = Cors::new()
        .allow_origins_fn(|_| true)
        .allow_methods(vec!["GET", "POST", "PUT", "DELETE", "PATCH"])
        .allow_credentials(true);

    let app = routes::routes()
        .at("/metrics", PrometheusExporter::new())
        .with(cors);

    Server::new(TcpListener::bind(("0.0.0.0", port)))
        .run(app)
        .await;
}