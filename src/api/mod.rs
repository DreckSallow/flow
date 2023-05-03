use std::net::SocketAddr;

use axum::{routing::get_service, Router, Server};
use tower_http::services::ServeDir;

#[tokio::main]
pub async fn run_api() {
    let app = Router::new().fallback_service(web_app_route());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server listen at: http://{addr}");
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn web_app_route() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./flow-ui/dist")))
}
