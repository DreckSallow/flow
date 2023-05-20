#[cfg(test)]
mod tests {
    use std::{fs, path::PathBuf};

    use crate::db::{project::project_utils, task::task_utils, Db};

    fn clean_db() {
        let mut test_path = Db::get_path_test_db();
        test_path.pop();
        fs::remove_dir_all(test_path).expect("Error cleaning the test db folder");
    }

    fn get_connection() -> Result<Db, rusqlite::Error> {
        Db::open_test_db()
    }

    #[test]
    fn get_one() {
        clean_db(); // Get the databse cleaned
        let db = get_connection().expect("Error open the database test");

        let project_res = project_utils::get_by_id(&db, 1);
        assert!(project_res.is_ok());

        let project = project_res.unwrap();
        assert_eq!(project.id, 1);
        assert_eq!(project.tasks.len(), 0);
    }

    #[test]
    fn test_project_tasks() {
        clean_db(); // Get the databse cleaned
        let db = get_connection().expect("Error open the database test");

        let task1_desc = ["Task 1", "Task2", "Learn Rust reading a book"];
        let new_project_id = project_utils::create_project(&db, PathBuf::from("./test-to-create"))
            .expect("Error creating new project")
            .id;

        for (i, d) in task1_desc.iter().enumerate() {
            task_utils::create_task(&db, d, new_project_id).expect("Error creating the task");
            let first_project = project_utils::get_by_id(&db, new_project_id).unwrap();
            assert_eq!(first_project.tasks.len(), (i + 1));
            assert!(first_project.tasks[i].temp_id == (i + 1) as u32);
        }
    }

    #[test]
    fn create_project_and_remove() {
        clean_db(); // Get the databse cleaned
        let db = get_connection().expect("Error open the database test");

        let mut project_ids = 1;

        let project_paths = vec!["./test-data", "./other-test-data"];

        for p in &project_paths {
            let project = project_utils::create_project(&db, PathBuf::from(p))
                .expect("Error creating a project");
            project_ids += 1;
            assert_eq!(project.id, project_ids);
            assert_eq!(project.path, PathBuf::from(p));
            assert_eq!(project.tasks.len(), 0);
        }

        let projects = project_utils::get_projects(&db).expect("Error getting all projects");
        assert_eq!(projects.len(), project_ids as usize);

        for i in 1..project_ids {
            let res = project_utils::remove_project(&db, i);
            println!("res: {:?}", res);
            assert!(res.is_ok())
        }

        let len = project_utils::get_projects(&db)
            .expect("Error getting the projects")
            .len();
        assert_eq!(len, 1)
    }
}
