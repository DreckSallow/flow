use crate::db::Db;

pub struct AppData {
    pub db: Db,
    pub current_project_id: u32,
}

impl AppData {
    pub fn new(db: Db, id: u32) -> Self {
        Self {
            db,
            current_project_id: id,
        }
    }
}
