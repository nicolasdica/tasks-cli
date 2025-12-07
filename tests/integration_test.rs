use std::fs;
use tasks_cli::Task;
use chrono::Utc;

fn create_test_tasks_file(file_path: &str) -> Vec<Task> {
    let tasks = vec![
        Task {
            name: String::from("Task 1"),
            date: Utc::now(),
            person: String::from("Nicolas"),
            completed: false,
            deleted: false,
        },
        Task {
            name: String::from("Task 2"),
            date: Utc::now(),
            person: String::from("Nicolas"),
            completed: false,
            deleted: false,
        },
        Task {
            name: String::from("Task 3"),
            date: Utc::now(),
            person: String::from("Nicolas"),
            completed: true,
            deleted: false,
        },
    ];

    let json = serde_json::to_string_pretty(&tasks).unwrap();
    fs::write(file_path, format!("{}\n", json)).unwrap();

    tasks
}

fn read_tasks_from_file(file_path: &str) -> Vec<Task> {
    let json_data = fs::read_to_string(file_path).unwrap();
    serde_json::from_str(&json_data).unwrap()
}

#[test]
fn test_mark_task_completed() {
    let test_file = "test_complete_task.json";

    create_test_tasks_file(test_file);

    tasks_cli::mark_task_completed(test_file, 0).unwrap();

    let tasks = read_tasks_from_file(test_file);
    assert!(tasks[0].completed, "Task 0 should be completed");
    assert!(!tasks[1].completed, "Task 1 should not be completed");
    assert_eq!(tasks[0].name, "Task 1");

    fs::remove_file(test_file).unwrap();
}

#[test]
fn test_mark_task_deleted() {
    let test_file = "test_delete_task.json";

    create_test_tasks_file(test_file);

    tasks_cli::mark_task_deleted(test_file, 1).unwrap();

    let tasks = read_tasks_from_file(test_file);
    assert!(!tasks[0].deleted, "Task 0 should not be deleted");
    assert!(tasks[1].deleted, "Task 1 should be deleted");
    assert!(!tasks[2].deleted, "Task 2 should not be deleted");
    assert_eq!(tasks[1].name, "Task 2");

    fs::remove_file(test_file).unwrap();
}

#[test]
fn test_mark_task_completed_invalid_index() {
    let test_file = "test_invalid_complete.json";

    create_test_tasks_file(test_file);

    let result = tasks_cli::mark_task_completed(test_file, 999);
    assert!(result.is_err(), "Should return error for invalid index");

    fs::remove_file(test_file).unwrap();
}

#[test]
fn test_mark_task_deleted_invalid_index() {
    let test_file = "test_invalid_delete.json";

    create_test_tasks_file(test_file);

    let result = tasks_cli::mark_task_deleted(test_file, 999);
    assert!(result.is_err(), "Should return error for invalid index");

    fs::remove_file(test_file).unwrap();
}

#[test]
fn test_complete_and_delete_same_task() {
    let test_file = "test_complete_and_delete.json";

    create_test_tasks_file(test_file);

    tasks_cli::mark_task_completed(test_file, 0).unwrap();
    tasks_cli::mark_task_deleted(test_file, 0).unwrap();

    let tasks = read_tasks_from_file(test_file);
    assert!(tasks[0].completed);
    assert!(tasks[0].deleted);

    fs::remove_file(test_file).unwrap();
}
