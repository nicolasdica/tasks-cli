use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::{thread, time};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub name: String,
    pub date: DateTime<Utc>,
    pub person: String,
    pub completed: bool,
    pub deleted: bool,
}

pub fn add_task() -> Result<(), Box<dyn std::error::Error>> {
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    let new_task = Task {
        name: name.trim().to_string(),
        date: Utc::now(),
        person: String::from("Nicolas"),
        completed: false,
        deleted: false,
    };

    let json_data = fs::read_to_string("tasks.json")?;
    let mut tasks: Vec<Task> = serde_json::from_str(&json_data)?;
    tasks.push(new_task);

    let updated_json = serde_json::to_string_pretty(&tasks)?;
    fs::write("tasks.json", format!("{}\n", updated_json))?;

    Ok(())
}

pub fn complete_task() -> Result<(), Box<dyn std::error::Error>> {
    let _ = list_tasks();

    println!("Which task do you want to mark as completed?");
    let mut task_index = String::new();
    io::stdin()
        .read_line(&mut task_index)
        .expect("Failed to read line");

    let index: usize = task_index.trim().parse()?;

    let _ = mark_task_completed("tasks.json", index - 1)?;

    println!("Task masked as complete.");

    Ok(())
}

pub fn mark_task_completed(
    file_path: &str,
    task_index: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    let json_data = fs::read_to_string(file_path)?;
    let mut tasks: Vec<Task> = serde_json::from_str(&json_data)?;

    if task_index >= tasks.len() {
        return Err(format!("Invalid task index: {}", task_index).into());
    }

    tasks[task_index].completed = true;

    let updated_json = serde_json::to_string_pretty(&tasks)?;
    fs::write(file_path, format!("{}\n", updated_json))?;

    Ok(())
}

pub fn delete_task() -> Result<(), Box<dyn std::error::Error>> {
    let _ = list_tasks();

    println!("Which task do you want to delete?");
    let mut task_index = String::new();
    io::stdin()
        .read_line(&mut task_index)
        .expect("Failed to read line");

    let index: usize = task_index.trim().parse()?;

    mark_task_deleted("tasks.json", index - 1)?;

    println!("Task deleted.");

    Ok(())
}

pub fn mark_task_deleted(
    file_path: &str,
    task_index: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    let json_data = fs::read_to_string(file_path)?;
    let mut tasks: Vec<Task> = serde_json::from_str(&json_data)?;

    if task_index >= tasks.len() {
        return Err(format!("Invalid task index: {}", task_index).into());
    }

    tasks[task_index].deleted = true;

    let updated_json = serde_json::to_string_pretty(&tasks)?;
    fs::write(file_path, format!("{}\n", updated_json))?;

    Ok(())
}

pub fn list_tasks() -> Result<(), Box<dyn std::error::Error>> {
    let json_data = fs::read_to_string("tasks.json")?;
    let tasks: Vec<Task> = serde_json::from_str(&json_data)?;

    for (index, task) in tasks.iter().enumerate() {
        println!("\n📝 {}", index + 1);
        println!("Name: {}", task.name);
        println!("Date: {}", task.date.format("%b %d, %Y %H:%M"));
        println!("Crated by: {}", task.person);
        println!(
            "Status: {}\n",
            if task.completed {
                "✅ Completed"
            } else {
                "⏳ Pending"
            }
        );

        thread::sleep(time::Duration::from_secs(1));
    }

    Ok(())
}
