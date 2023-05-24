use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use serde_json::json;

use crate::AppModels;

async fn get_all_projects(State(app): State<Arc<AppModels>>) -> impl IntoResponse {
    let projects_res = app.project.find_all().await;
    match projects_res {
        Ok(p) => (StatusCode::OK, Json(json!({ "data":p,"message":""}))),
        Err(e) => {
            println!("ERROR: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "data":None::<bool>,"message":e.to_string()})),
            )
        }
    }
}

async fn get_tasks_by_project(
    Path(project_id): Path<u32>,
    State(app): State<Arc<AppModels>>,
) -> impl IntoResponse {
    let tasks_res = app.task.find_by_project(project_id).await;

    match tasks_res {
        Ok(tasks) => {
            let message = if tasks.is_empty() {
                "No have tasks"
            } else {
                ""
            };

            (
                StatusCode::OK,
                Json(json!({ "data":tasks,"message":message})),
            )
        }
        Err(e) => {
            println!("ERROR: {}", e);

            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "data":None::<bool>,"message":e.to_string()})),
            )
        }
    }
}

async fn get_all_tasks(State(app): State<Arc<AppModels>>) -> impl IntoResponse {
    let tasks_res = app.task.find_all().await;

    match tasks_res {
        Ok(tasks) => {
            let message = if tasks.is_empty() {
                "No have tasks"
            } else {
                ""
            };

            (
                StatusCode::OK,
                Json(json!({ "data":tasks,"message":message})),
            )
        }
        Err(e) => {
            println!("ERROR: {}", e);

            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "data":None::<bool>,"message":e.to_string()})),
            )
        }
    }
}

pub fn routes(state: Arc<AppModels>) -> Router {
    Router::new()
        .route("/projects", get(get_all_projects))
        .route("/tasks", get(get_all_tasks))
        .route("/tasks/:project_id", get(get_tasks_by_project))
        .with_state(state)
}
