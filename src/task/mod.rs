pub mod db_model;
mod task;
pub use task::Task;

use crate::{
    app_data::AppData,
    utils::table::{RowCell, Table},
};

use db_model::TaskModelUtils;

#[derive(Debug)]
pub struct TaskProgram;

impl TaskProgram {
    pub fn run_default(app_data: &AppData, desc: &str) {
        match TaskModelUtils::create_task(&app_data.db, desc, app_data.current_project_id) {
            Ok(_) => {
                println!("Task '{}' created!", desc);
            }
            Err(e) => {
                eprintln!("Error creating the task: {}", e);
            }
        }
    }
    pub fn run_list(app_data: &AppData, expand: bool, order_by: Option<String>) {
        if let Some(mut tasks) = TaskProgram::get_tasks(app_data) {
            let mut table = Table::new();
            let tasks_len = tasks.len();

            if let Some(order) = order_by {
                match order.as_ref() {
                    "number" => {
                        tasks.sort_by(|t1, t2| t1.temp_id.cmp(&t2.temp_id));
                    }
                    "desc" => {
                        tasks.sort_by(|t1, t2| t1.description.cmp(&t2.description));
                    }
                    _ => {}
                }
            }

            if expand {
                table.add_headers(vec!["N-id", "Description", "Id"]);
            } else {
                table.add_headers(vec!["N-id", "Description"]);
            }

            for task in tasks {
                let id = RowCell::Single(task.temp_id.to_string());
                let desc = RowCell::Single(task.description.clone());
                let row = if expand {
                    vec![id, desc, RowCell::Single(task.id.to_string())]
                } else {
                    vec![id, desc]
                };

                table.insert_row(row);
            }
            if tasks_len > 0 {
                println!("{}", table.get_table(1));
            } else {
                println!("You don't have any tasks.");
            }

            println!("Total Tasks: {}", tasks_len);
        }
    }

    pub fn run_delete(app_data: &AppData, task_temp_id: u32) {
        if let Some(tasks) = TaskProgram::get_tasks(app_data) {
            let task_id = tasks.iter().find_map(|task| {
                if task.temp_id == task_temp_id {
                    Some(task.id)
                } else {
                    None
                }
            });

            match task_id {
                Some(id) => match TaskModelUtils::remove_task(&app_data.db, id) {
                    Ok(_) => {
                        println!("Task '{}' deleted", task_temp_id);
                    }
                    Err(e) => eprintln!("Error removing the task : {}", e),
                },

                None => {
                    eprintln!("The task id {} not found", task_temp_id);
                }
            }
        }
    }

    fn get_tasks(app_data: &AppData) -> Option<Vec<Task>> {
        match TaskModelUtils::get_tasks(&app_data.db, app_data.current_project_id) {
            Ok(tasks) => Some(tasks),
            Err(e) => {
                eprintln!("Error getting the tasks: {}", e);
                None
            }
        }
    }
}
