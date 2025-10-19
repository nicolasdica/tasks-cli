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
        io::stdin().read_line(&mut option).expect("Failed to read line");

        let _ = match option.trim() {
            "1" => add_task(),
            "2" => list_tasks(),
            "3" => complete_task(),
            "4" => delete_task(),
            "q" => {
                println!("Exiting program...");
                thread::sleep(time::Duration::from_secs(1));

                break;
            },
            _ => todo!()
        };
    }
}
