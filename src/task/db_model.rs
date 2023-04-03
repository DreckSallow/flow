use chrono::Local;
use rusqlite::Result;

use crate::db::Db;

use super::{Task, TaskStatus};

pub struct TaskModelUtils;

impl TaskModelUtils {
    pub fn create_task(db: &Db, desc: &str, project_id: u32) -> Result<u32> {
        db.0.execute(
            "INSERT INTO tasks (description,date,project_id,status) values (?1,?2,?3,?4)",
            [
                desc.to_string(),
                Local::now().to_string(),
                project_id.to_string(),
                TaskStatus::NoStarted.to_string(),
            ],
        )?;
        Ok(db.0.last_insert_rowid() as u32)
    }

    pub fn get_tasks(db: &Db, project_id: u32) -> Result<Vec<Task>> {
        let mut stmt =
            db.0.prepare("SELECT * FROM tasks WHERE project_id = ? ORDER BY id ASC")?; // Order by Id for temp_id

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
            "UPDATE tasks SET status = ?1 WHERE id = ?2 ",
            [status.to_string(), id.to_string()],
        )?;

        Ok(())
    }

    pub fn remove_task(db: &Db, id: u32) -> Result<()> {
        db.0.execute("DELETE FROM tasks WHERE id = ?1", [id])?;
        Ok(())
    }
}
