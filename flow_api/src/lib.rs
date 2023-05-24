mod routes;

use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use axum::{http::Method, routing::get_service, Router, Server};
use flow_data::db::{project::ProjectModelAsync, task::TaskModelAsync, Db};
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
};

pub struct AppModels {
    pub project: ProjectModelAsync,
    pub task: TaskModelAsync,
}

impl AppModels {
    pub fn create() -> Self {
        let db = Arc::new(Mutex::new(Db::open().unwrap()));
        let project = ProjectModelAsync::new(&db);
        let task = TaskModelAsync::new(&db);

        Self { project, task }
    }
}

#[tokio::main]
pub async fn run_api() {
    let app_state = Arc::new(AppModels::create());

    let app = Router::new()
        .fallback_service(web_app_route())
        .nest("/api", routes::routes(app_state))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods([Method::GET]),
        );

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
