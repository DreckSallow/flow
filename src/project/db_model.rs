use std::{collections::HashMap, path::PathBuf};

use rusqlite::Result;

use crate::{db::Db, task::Task};

use super::Project;

pub struct ProjectModel<'a> {
    db: &'a Db,
}

impl<'a> ProjectModel<'a> {
    pub fn get_projects(self) -> Result<HashMap<String, Project>> {
        ProjectModelUtils::get_projects_db(self.db)
    }
    pub fn create_project(self, path: PathBuf) -> Result<Project> {
        ProjectModelUtils::create_project(self.db, &path)
    }
}

pub struct ProjectModelUtils;

impl ProjectModelUtils {
    pub fn get_projects_db(db: &Db) -> Result<HashMap<String, Project>> {
        let mut s_1 = db
            .0
            .prepare("
                SELECT p.id, p.path, t.id, t.description, t.date FROM projects p LEFT JOIN tasks t ON t.project_id = p.id
            ")?;

        let _ = db.0.prepare("SELECT * FROM projects")?.query_map((), |r| {
            let id: u32 = r.get(0)?;
            Ok(())
        })?;

        let mut hash_projects: HashMap<String, Project> = HashMap::new();

        let _ = s_1.query_map((), |row| {
            let project_id: u32 = row.get(0)?;
            let project_path: String = row.get(1)?;
            let task_id: Result<u32> = row.get(2);
            let task_desc: Result<String> = row.get(3);
            if hash_projects.contains_key(&project_path) && task_id.is_ok() {
                hash_projects
                    .get_mut(&project_path)
                    .unwrap()
                    .tasks
                    .push(Task::new(task_id.unwrap(), &task_desc.unwrap()));
            } else {
                hash_projects.insert(
                    project_path.clone(),
                    Project::new(project_id, PathBuf::from(project_path), vec![]),
                );
            }
            Ok(())
        })?;
        Ok(hash_projects)
    }
    pub fn create_project(db: &Db, path: &PathBuf) -> Result<Project> {
        db.0.execute("INSERT INTO projects (path) values (?1)", [path.to_str()])?;
        let id: u32 = db.0.last_insert_rowid().to_string().parse().unwrap();
        Ok(Project::new(id, path.to_owned(), vec![]))
    }
}
