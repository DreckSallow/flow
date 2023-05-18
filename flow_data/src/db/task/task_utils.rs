use chrono::Local;
use rusqlite::Result;

use crate::db::Db;

use crate::db::queries::task_query;
use crate::task::{Task, TaskStatus};

pub fn create_task(db: &Db, desc: &str, project_id: u32) -> Result<u32> {
    db.0.execute(
        task_query::CREATE_TASK,
        [
            desc.to_string(),
            Local::now().to_string(),
            project_id.to_string(),
            TaskStatus::NoStarted.to_string(),
        ],
    )?;
    Ok(db.0.last_insert_rowid() as u32)
}

pub fn get_tasks_by_project(db: &Db, project_id: u32) -> Result<Vec<Task>> {
    let mut stmt = db.0.prepare(task_query::TASKS_BY_PROJECT)?; // Order by Id for temp_id

    let rows = stmt.query_map([project_id], |row| {
        let id: u32 = row.get(0)?;
        let desc: String = row.get(1)?;
        let status: String = row.get(3)?;
        // TODO: Get the date:
        // let date: String = row.get(2)?;
        Ok((id, desc, TaskStatus::from(status)))
    })?;

    Ok(rows
        .enumerate()
        .filter_map(|(i, r)| match r {
            Ok((id, desc, status)) => Some(Task::new(id, &desc, (i + 1) as u32, status)),
            Err(_) => None,
        })
        .collect())
}

pub fn update_task_status(db: &Db, id: u32, status: &TaskStatus) -> Result<()> {
    db.0.execute(
        task_query::UPDATE_STATUS,
        [status.to_string(), id.to_string()],
    )?;

    Ok(())
}

pub fn remove_task(db: &Db, id: u32) -> Result<()> {
    db.0.execute(task_query::DELETE_TASK, [id])?;
    Ok(())
}
