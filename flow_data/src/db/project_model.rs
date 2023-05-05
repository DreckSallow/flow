use std::{collections::HashMap, path::PathBuf};

use rusqlite::Result;

use crate::{
    db::Db,
    task::{Task, TaskStatus},
};

use crate::project::Project;

pub struct ProjectModel<'a> {
    db: &'a Db,
}

#[allow(dead_code)]
impl<'a> ProjectModel<'a> {
    pub fn get_projects(self) -> Result<HashMap<String, Project>> {
        ProjectModelUtils::get_projects(self.db)
    }
    pub fn create_project(self, path: PathBuf) -> Result<Project> {
        ProjectModelUtils::create_project(self.db, &path)
    }
}

pub struct ProjectModelUtils;

impl ProjectModelUtils {
    pub fn get_projects(db: &Db) -> Result<HashMap<String, Project>> {
        let mut s_1 = db
            .0
            .prepare("
                SELECT p.id, p.path, t.id, t.description, t.date, t.status from projects p LEFT JOIN tasks t ON t.project_id = p.id
            ")?;

        let mut hash_projects: HashMap<String, Project> = HashMap::new();

        let mapped = s_1.query_map((), |row| {
            let project_id: u32 = row.get(0)?;
            let project_path: String = row.get(1)?;

            let task_id: Result<u32> = row.get(2);
            let task_desc: Result<String> = row.get(3);
            let task_status: Result<String> = row.get(5);
            let project = Project::new(project_id, PathBuf::from(project_path), vec![]);

            if task_id.is_ok() && task_desc.is_ok() && task_status.is_ok() {
                return Ok((
                    project,
                    Some((task_id.unwrap(), task_desc.unwrap(), task_status.unwrap())),
                ));
            }

            Ok((project, None))
        })?;

        {
            let mut temp_tasks: HashMap<u32, Vec<(u32, String, String)>> = HashMap::new();
            let mut projects = HashMap::new();

            mapped.for_each(|r| {
                if let Err(_) = r {
                    return;
                }
                let (project, task_data) = r.unwrap();

                //Group tasks by project_id:
                if temp_tasks.contains_key(&project.id) && task_data.is_some() {
                    temp_tasks
                        .get_mut(&project.id)
                        .unwrap()
                        .push(task_data.unwrap())
                } else {
                    if let Some(t) = task_data {
                        temp_tasks.insert(project.id, vec![t]);
                    } else {
                        temp_tasks.insert(project.id, vec![]);
                    }
                }

                //Save projects temporary by path:
                if !projects.contains_key(&project.path.display().to_string()) {
                    projects.insert(project.path.display().to_string(), project);
                }
            });

            for (key, mut p) in projects {
                // Created the tasks filtered
                let tasks: Vec<Task> = temp_tasks
                    .get(&p.id)
                    .unwrap()
                    .iter()
                    .enumerate()
                    .map(|(i, t)| {
                        Task::new(t.0, &t.1, (i + 1) as u32, TaskStatus::from(t.2.clone()))
                    })
                    .collect();

                p.tasks = tasks;
                hash_projects.insert(key, p);
            }
        }

        Ok(hash_projects)
    }
    pub fn create_project(db: &Db, path: &PathBuf) -> Result<Project> {
        db.0.execute("INSERT INTO projects (path) values (?1)", [path.to_str()])?;
        let id: u32 = db.0.last_insert_rowid().to_string().parse().unwrap();
        Ok(Project::new(id, path.to_owned(), vec![]))
    }

    pub fn remove_project(db: &Db, id: u32) -> Result<()> {
        db.0.execute("DELETE FROM projects WHERE id = ?1", [id])?;
        Ok(())
    }

    pub fn get_by_id(db: &Db, id: u32) -> Result<Project> {
        let mut stmt = db.0.prepare("SELECT * FROM projects WHERE id = ?1")?;
        stmt.query_row([id], |row| {
            let path: String = row.get(1)?;
            Ok(Project::new(row.get(0)?, PathBuf::from(path), vec![]))
        })
    }
}
