use std::fs;
use std::io;
use chrono::{DateTime, Utc};
use std::{thread, time};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub name: String,
    pub date: DateTime<Utc>,
    pub person: String,
    pub completed: bool,
    pub deleted: bool,
}

pub fn add_task() {
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read line");

    let new_task = Task {
        name: name.trim().to_string(),
        date: Utc::now(),
        person: String::from("Nicolas"),
        completed: false,
        deleted: false,
    };

    let json_data = match fs::read_to_string("tasks.json") {
        Ok(data) => data,
        Err(e) => {
            println!("Error reading file tasks.json: {}", e);
            return;
        }
    };

    let mut tasks: Vec<Task> = serde_json::from_str(&json_data).unwrap();
    tasks.push(new_task);

    let updated_json = serde_json::to_string_pretty(&tasks).unwrap();
    fs::write("tasks.json", format!("{}\n", updated_json)).unwrap();
}

pub fn complete_task() {
    list_tasks();

    println!("Which task do you want to mark as completed?");
    let mut task_index = String::new();
    io::stdin().read_line(&mut task_index).expect("Failed to read line");

    let index: usize = match task_index.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input! Please enter a valid number.");
            return;
        }
    };

    let json_data = match fs::read_to_string("tasks.json") {
        Ok(data) => data,
        Err(e) => {
            println!("Error reading file tasks.json: {}", e);
            return;
        }
    };
    let mut tasks: Vec<Task> = serde_json::from_str(&json_data).unwrap();

    if index >= tasks.len() {
        println!("Invalid task {}", index);
        return;
    }

    tasks[index].completed = true;

    let updated_json = serde_json::to_string_pretty(&tasks).unwrap();
    fs::write("tasks.json", format!("{}\n", updated_json)).unwrap();
}

pub fn delete_task() {
    list_tasks();

    println!("Which task do you want to delete?");
    let mut task_index = String::new();
    io::stdin().read_line(&mut task_index).expect("Failed to read line");

    let index: usize = match task_index.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input! Please enter a valid number.");
            return;
        }
    };

    let json_data = match fs::read_to_string("tasks.json") {
        Ok(data) => data,
        Err(e) => {
            println!("Error reading file tasks.json: {}", e);
            return;
        }
    };
    let mut tasks: Vec<Task> = serde_json::from_str(&json_data).unwrap();

    if index >= tasks.len() {
        println!("Invalid task {}", index);
        return;
    }

    tasks[index].deleted = true;

    let updated_json = serde_json::to_string_pretty(&tasks).unwrap();
    fs::write("tasks.json", format!("{}\n", updated_json)).unwrap();
}

pub fn list_tasks() {
    let json_data = fs::read_to_string("tasks.json").unwrap();
    let tasks: Vec<Task> = serde_json::from_str(&json_data).unwrap();

    for (index, task) in tasks.iter().enumerate() {
        println!("\nTask {}", index);
        println!("Name: {}", task.name);
        println!("Date: {:?}", task.date);
        println!("Person: {}", task.person);
        println!("Completed: {}\n", task.completed);

        thread::sleep(time::Duration::from_secs(1));
    }
}
