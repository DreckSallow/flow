pub mod db_model;
mod task;

pub use task::{Task, TaskStatus};

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
                table.add_headers(vec!["N-id", "Description", "Status", "Id"]);
            } else {
                table.add_headers(vec!["N-id", "Description", "Status"]);
            }

            for task in tasks {
                let id = RowCell::Single(task.temp_id.to_string());
                let desc = RowCell::Single(task.description.clone());
                let status = RowCell::Single(task.status.to_string());
                let row = if expand {
                    vec![id, desc, status, RowCell::Single(task.id.to_string())]
                } else {
                    vec![id, desc, status]
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

    pub fn run_update_status(app_data: &AppData, tasks_temp_ids: Vec<u32>, status: TaskStatus) {
        match TaskProgram::get_tasks(app_data) {
            Some(tasks) => {
                let task_ids: Vec<u32> = tasks
                    .iter()
                    .filter_map(|t| {
                        if tasks_temp_ids.contains(&t.temp_id) {
                            Some(t.id)
                        } else {
                            None
                        }
                    })
                    .collect();
                if task_ids.len() == 0 {
                    return eprintln!("Task not found! :(");
                }

                let errors: Vec<_> = task_ids
                    .iter()
                    .map(|id| TaskModelUtils::update_task_status(&app_data.db, *id, &status))
                    .filter_map(|r| r.err())
                    .collect();

                if errors.len() > 0 {
                    eprintln!("Error updating tasks");
                    for e in errors {
                        eprintln!("{}", e);
                    }
                } else {
                    println!("Tasks updated");
                }
            }
            None => {
                println!("You don't have any tasks.");
            }
        }
    }

    pub fn run_do_task(app_data: &AppData, tasks_temp_id: u32) {
        if let Some(tasks) = TaskProgram::get_tasks(app_data) {
            let find_task = tasks.iter().find(|t| t.temp_id == tasks_temp_id);
            if let None = find_task {
                return eprintln!("Task not found! :(");
            }

            // Check if exist another task with the status = 'In Progress':
            let exist_other = tasks
                .iter()
                .find(|t| t.temp_id != tasks_temp_id && t.status == TaskStatus::Start)
                .is_some();

            if exist_other {
                return eprintln!(
                    "There is another task 'In Progress', complete the previous task!"
                );
            }
            let task = find_task.unwrap();
            match TaskModelUtils::update_task_status(&app_data.db, task.id, &TaskStatus::Start) {
                Ok(_) => {
                    println!("Task updated!")
                }
                Err(e) => eprintln!("Error updating task: {}", e),
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
