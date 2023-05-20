use anyhow::{anyhow, Result};
use crossterm::style::{Color, Stylize};
use flow_data::{
    app_data::AppData,
    db::task::task_utils,
    task::{Task, TaskStatus},
};

use crate::utils::table::{RowCell, Table};

fn get_tasks(app_data: &AppData) -> Result<Vec<Task>> {
    let tasks = task_utils::get_tasks_by_project(&app_data.db, app_data.current_project_id)?;
    Ok(tasks)
}

pub fn create_task(app_data: &AppData, desc: &str) -> Result<()> {
    task_utils::create_task(&app_data.db, desc, app_data.current_project_id)?;
    println!("Task '{}' created!", desc);
    Ok(())
}

pub fn delete_one(app_data: &AppData, task_temp_id: u32) -> Result<()> {
    let tasks = get_tasks(app_data)?;
    let task_id = tasks.iter().find_map(|task| {
        if task.temp_id == task_temp_id {
            Some(task.id)
        } else {
            None
        }
    });

    match task_id {
        Some(id) => {
            task_utils::remove_task(&app_data.db, id)?;
            println!("Task '{}' deleted", task_temp_id);
            Ok(())
        }

        None => Err(anyhow!("The task id {} not found", task_temp_id)),
    }
}

pub fn update_task_status(
    app_data: &AppData,
    task_temp_ids: Vec<u32>,
    status: TaskStatus,
) -> Result<()> {
    let tasks = get_tasks(app_data)?;

    let task_ids: Vec<u32> = tasks
        .iter()
        .filter_map(|t| task_temp_ids.contains(&t.temp_id).then(|| t.id))
        .collect();

    if task_ids.len() == 0 {
        return Err(anyhow!("Task not found! :("));
    }

    let errors: Vec<_> = task_ids
        .iter()
        .map(|id| task_utils::update_task_status(&app_data.db, *id, &status))
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
    Ok(())
}

pub fn start_one_task(app_data: &AppData, tasks_temp_id: u32) -> Result<()> {
    let tasks = get_tasks(app_data)?;
    let find_task = tasks.iter().find(|t| t.temp_id == tasks_temp_id);

    if find_task.is_none() {
        return Err(anyhow!("Task not found! :("));
    }

    // Check if exist another task with the status = 'In Progress':
    let exist_other = tasks
        .iter()
        .find(|t| t.temp_id != tasks_temp_id && t.status == TaskStatus::Start)
        .is_some();

    if exist_other {
        return Err(anyhow!(
            "There is another task 'In Progress', complete the previous task!"
        ));
    }

    let task = find_task.unwrap();
    task_utils::update_task_status(&app_data.db, task.id, &TaskStatus::Start)?;
    println!("Task updated!");
    Ok(())
}

pub fn list_tasks(
    app_data: &AppData,
    expand: bool,
    order_by: Option<String>,
    with_color: bool,
) -> Result<()> {
    let mut tasks = get_tasks(app_data)?;
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
        let (id, desc, status) = {
            let color = status_to_color(&task.status);

            if !with_color || color.is_none() {
                (
                    RowCell::Single(task.temp_id.to_string()),
                    RowCell::Single(task.description.clone()),
                    RowCell::Single(task.status.to_string()),
                )
            } else {
                let v = styled_attrs(
                    vec![
                        &task.temp_id.to_string(),
                        &task.description.to_string(),
                        &task.status.to_string(),
                    ],
                    color.unwrap_or(Color::DarkGrey),
                );

                (
                    RowCell::Styled(v[0].0.clone(), v[0].1.clone()),
                    RowCell::Styled(v[1].0.clone(), v[1].1.clone()),
                    RowCell::Styled(v[2].0.clone(), v[2].1.clone()),
                )
            }
        };

        let mut row = vec![id, desc, status];

        if expand {
            let mut task_id = RowCell::Single(task.id.to_string());
            if with_color {
                let color = status_to_color(&task.status);

                if let Some(c) = color {
                    task_id = RowCell::Styled(
                        task.id.to_string().with(c).to_string(),
                        task.id.to_string(),
                    );
                }
            }
            row.push(task_id);
        }

        table.insert_row(row);
    }
    if tasks_len > 0 {
        println!("{}", table.get_table(1));
    } else {
        println!("You don't have any tasks.");
    }

    println!("Total Tasks: {}", tasks_len);
    Ok(())
}

fn styled_attrs(texts: Vec<&str>, color: Color) -> Vec<(String, String)> {
    let mut v = vec![];

    for s in texts {
        v.push((s.with(color).to_string(), s.to_owned()))
    }

    v
}
fn status_to_color(status: &TaskStatus) -> Option<Color> {
    match status {
        TaskStatus::Start => Some(Color::Green),
        TaskStatus::Stop => Some(Color::Blue),
        TaskStatus::Done => Some(Color::DarkYellow),
        TaskStatus::NoStarted => None,
    }
}
