use std::{collections::HashMap, path::PathBuf};

use rusqlite::Result;

use crate::{
    db::{queries::project_query, Db},
    project::Project,
    task::{Task, TaskStatus},
};

pub fn get_projects(db: &Db) -> Result<HashMap<String, Project>> {
    let mut s_1 = db.0.prepare(project_query::ALL_PROJECTS)?;

    let mut projects_store: HashMap<String, Project> = HashMap::new();

    let mapped = s_1.query_map((), |row| {
        let project_id: u32 = row.get(0)?;
        let project_path: String = row.get(1)?;

        let task_id: Result<u32> = row.get(2);
        let task_desc: Result<String> = row.get(3);
        let task_status: Result<String> = row.get(5);

        let project = Project::new(project_id, project_path.into(), vec![]);

        if task_id.is_ok() && task_desc.is_ok() && task_status.is_ok() {
            return Ok((
                project,
                Some((task_id.unwrap(), task_desc.unwrap(), task_status.unwrap())),
            ));
        }

        Ok((project, None))
    })?;

    mapped.for_each(|data| {
        if data.is_err() {
            return;
        }
        let (project, task_info) = data.unwrap();

        match projects_store.get_mut(&project.path.display().to_string()) {
            Some(project_saved) => {
                if let Some((id, desc, status_text)) = task_info {
                    if let None = project_saved.tasks.iter().find(|t| t.id == id) {
                        project_saved.tasks.push(Task::new(
                            id,
                            &desc,
                            project_saved.tasks.len() as u32,
                            TaskStatus::from(status_text),
                        ))
                    }
                }
            }
            None => {
                projects_store.insert(project.path.display().to_string(), project);
            }
        }
    });

    Ok(projects_store)
}

pub fn create_project(db: &Db, path: PathBuf) -> Result<Project> {
    db.0.execute(project_query::CREATE_PROJECT, [path.to_str()])?;
    let id: u32 = db.0.last_insert_rowid().to_string().parse().unwrap();
    Ok(Project::new(id, path.to_owned(), vec![]))
}
pub fn remove_project(db: &Db, id: u32) -> Result<()> {
    db.0.execute(project_query::DELETE_PROJECT, [id])?;
    Ok(())
}

pub fn get_by_id(db: &Db, id: u32) -> Result<Project> {
    let mut stmt = db.0.prepare(project_query::PROJECT_BY_ID)?;

    let mapped = stmt
        .query_map([id], |row| {
            let id: u32 = row.get(0)?;
            let path: String = row.get(1)?;
            let (t_id, t_desc, t_date, t_status): (
                Result<u32>,
                Result<String>,
                Result<String>,
                Result<String>,
            ) = (row.get(2), row.get(3), row.get(4), row.get(4));

            if t_id.is_ok() && t_desc.is_ok() && t_date.is_ok() && t_status.is_ok() {
                return Ok((
                    (id, path),
                    Some((
                        t_id.unwrap(),
                        t_desc.unwrap(),
                        t_date.unwrap(),
                        t_status.unwrap(),
                    )),
                ));
            }
            Ok(((id, path), None))
        })?
        .filter_map(|r| r.ok())
        .enumerate();

    let mut project: Option<Project> = None;

    for (i, (project_data, task_opt)) in mapped {
        if let None = project {
            project = Some(Project::new(project_data.0, project_data.1.into(), vec![]));
        }

        if let Some(ref mut p) = project {
            if let Some((id, desc, _, status)) = task_opt {
                p.tasks.push(Task::new(
                    id,
                    &desc,
                    (i + 1) as u32,
                    TaskStatus::from(status),
                ));
            }
        }
    }
    Ok(project.unwrap())
}
