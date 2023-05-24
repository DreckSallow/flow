pub mod project_query {
    /// #### Query Return
    /// `Project ID` | `Project PATH` |
    /// `Task ID`  | `Task DESC`  | `Task Date` | `Task STATUS`
    pub const ALL_PROJECTS: &str = r#"
      SELECT p.id, p.path, t.id, t.description, t.date, t.status
      FROM projects p 
      LEFT JOIN tasks t
      ON t.project_id = p.id
    "#;

    /// #### Query Return
    /// `Project ID` | `Project PATH` |
    /// `Task ID`  | `Task DESC`  | `Task Date` | `Task STATUS`
    /// #### ARGS
    /// `Project ID`
    pub const PROJECT_BY_ID: &str = r#"
      SELECT p.id, p.path, t.id, t.description, t.date, t.status    
      FROM projects p
      LEFT JOIN tasks t ON t.project_id = p.id
      WHERE p.id = ?1
    "#;

    pub const CREATE_PROJECT: &str = "INSERT INTO projects (path) values (?1)";

    pub const DELETE_PROJECT: &str = "DELETE FROM projects WHERE id = ?1";
}

pub mod task_query {
    /// #### Query Return
    /// `Task ID` | `Task Desc` | `Task Date` | `Task Status`
    /// #### ARGS
    /// `Project ID`
    pub const TASKS_BY_PROJECT: &str = "SELECT * FROM tasks WHERE project_id = ? ORDER BY id ASC";

    /// #### Query Return
    /// `Task ID` | `Task Desc` | `Task Date` | `Task Status`
    pub const ALL_TASKS: &str = "SELECT * FROM tasks ORDER BY id ASC";

    /// #### ARGS
    /// `Task Status` |`Task ID`
    pub const UPDATE_STATUS: &str = "UPDATE tasks SET status = ?1 WHERE id = ?2 ";

    /// #### ARGS
    /// `Description` | `Date` | `Project ID` | `Status`
    pub const CREATE_TASK: &str =
        "INSERT INTO tasks (description,date,project_id,status) values (?1,?2,?3,?4)";

    /// #### ARGS
    /// `Task ID`
    pub const DELETE_TASK: &str = "DELETE FROM tasks WHERE id = ?1";
}
