use std::path::PathBuf;

use rusqlite::Result;

use crate::project::Project;

use super::Db;

pub mod project_utils;

pub struct ProjectModel<'a> {
    db: &'a Db,
}

impl<'a> ProjectModel<'a> {
    pub fn new(db: &'a Db) -> Self {
        Self { db }
    }
    pub fn find_by_id(&self, id: u32) -> Result<Project> {
        project_utils::get_by_id(self.db, id)
    }

    pub fn create<P: Into<PathBuf>>(&self, p: P) -> Result<Project> {
        project_utils::create_project(self.db, PathBuf::from(p.into()))
    }

    pub fn find_all(&self) -> Result<Vec<Project>> {
        let projects_hash = project_utils::get_projects(self.db)?;
        let mut projects = vec![];
        for (_, p) in projects_hash {
            projects.push(p)
        }
        Ok(projects)
    }

    pub fn delete_by_id(&self, id: u32) -> Result<()> {
        project_utils::remove_project(self.db, id)
    }
}

pub struct ProjectModelAsync<'a> {
    db: &'a Db,
}

impl<'a> ProjectModelAsync<'a> {
    pub fn new(db: &'a Db) -> Self {
        Self { db }
    }

    pub async fn find_by_id(&self, id: u32) -> Result<Project> {
        project_utils::get_by_id(self.db, id)
    }

    pub async fn create<P: Into<PathBuf>>(&self, p: P) -> Result<Project> {
        project_utils::create_project(self.db, PathBuf::from(p.into()))
    }

    pub async fn find_all(&self) -> Result<Vec<Project>> {
        let projects_hash = project_utils::get_projects(self.db)?;
        let mut projects = vec![];
        for (_, p) in projects_hash {
            projects.push(p)
        }
        Ok(projects)
    }

    pub async fn delete_by_id(&self, id: u32) -> Result<()> {
        project_utils::remove_project(self.db, id)
    }
}
