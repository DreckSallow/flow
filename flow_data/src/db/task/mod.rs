use std::sync::{Arc, Mutex};

use rusqlite::Result;

use crate::task::{Task, TaskStatus};

use super::Db;

pub mod task_utils;

pub struct TaskModel<'a> {
    db: &'a Db,
}

impl<'a> TaskModel<'a> {
    pub fn new(db: &'a Db) -> Self {
        Self { db }
    }

    pub fn create(&self, desc: &str, project_id: u32) -> Result<u32> {
        task_utils::create_task(self.db, desc, project_id)
    }

    pub fn find_by_project(&self, project_id: u32) -> Result<Vec<Task>> {
        task_utils::get_tasks_by_project(self.db, project_id)
    }

    pub fn delete_by_id(&self, id: u32) -> Result<()> {
        task_utils::remove_task(self.db, id)
    }

    pub fn update_status(&self, id: u32, status: TaskStatus) -> Result<()> {
        task_utils::update_task_status(self.db, id, &status)
    }
}

#[derive(Clone)]
pub struct TaskModelAsync {
    db: Arc<Mutex<Db>>,
}

impl TaskModelAsync {
    pub fn new(db: &Arc<Mutex<Db>>) -> Self {
        Self {
            db: Arc::clone(&db),
        }
    }

    pub async fn create(&self, desc: &str, project_id: u32) -> Result<u32> {
        task_utils::create_task(&self.db.lock().unwrap(), desc, project_id)
    }

    pub async fn find_by_project(&self, project_id: u32) -> Result<Vec<Task>> {
        task_utils::get_tasks_by_project(&self.db.lock().unwrap(), project_id)
    }

    pub async fn delete_by_id(&self, id: u32) -> Result<()> {
        task_utils::remove_task(&self.db.lock().unwrap(), id)
    }

    pub async fn update_status(&self, id: u32, status: TaskStatus) -> Result<()> {
        task_utils::update_task_status(&self.db.lock().unwrap(), id, &status)
    }
}
