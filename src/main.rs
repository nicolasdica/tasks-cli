use std::io;
use std::{thread, time};
use tasks_cli::{add_task, complete_task, delete_task, list_tasks};

fn main() {
    loop {
        println!("\nPlease select an option:");
        println!("=> 1 - Add task");
        println!("=> 2 - List all tasks");
        println!("=> 3 - Mark task as completed");
        println!("=> 4 - Delete task");
        println!("Press q to exit\n");

        let mut option = String::new();
        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read line");

        let _ = match option.trim() {
            "1" => {
                if let Err(e) = add_task() {
                    eprintln!("Error adding the task: {}", e);
                }
            }
            "2" => {
                if let Err(e) = list_tasks() {
                    eprintln!("Error listing tasks: {}", e);
                }
            }
            "3" => {
                if let Err(e) = complete_task() {
                    eprintln!("Error completing task: {}", e);
                }
            }
            "4" => {
                if let Err(e) = delete_task() {
                    eprintln!("Error deleting task: {}", e);
                }
            }
            "q" => {
                println!("Exiting program...");
                thread::sleep(time::Duration::from_secs(1));

                break;
            }
            _ => {
                println!("Invalid option! Please try again.")
            }
        };

        thread::sleep(time::Duration::from_secs(1));
    }
}
