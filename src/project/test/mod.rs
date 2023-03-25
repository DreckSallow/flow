#[cfg(test)]
mod tests {
    use std::{fs, io};

    use rusqlite::Result;

    use super::super::db_model::ProjectModelUtils;
    use crate::{
        db::Db,
        project::Project,
        utils::{self, test_utils::After},
    };

    fn create_project(dirname: &str, conn: &Db) -> Result<Project> {
        let mut current_path = utils::get_current_directory().unwrap();
        current_path.push(dirname);

        ProjectModelUtils::create_project(&conn, &current_path)
    }

    fn remove_db_test() -> io::Result<()> {
        let db_test_path = Db::get_path_test_db();
        println!("db test: {:?}", db_test_path);
        fs::remove_file(db_test_path)
    }

    #[test]
    fn test_create_project() {
        let _after = After(|| {
            if let Err(e) = remove_db_test() {
                println!("Error removing db for tests: {}", e);
            }
        });
        let conn = Db::open_test_db().unwrap();
        let project_res = create_project("../dir-test", &conn);
        assert!(project_res.is_ok());

        let project = project_res.unwrap();

        assert_eq!(project.id, 1);
    }
}
