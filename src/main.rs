use std::io::{self, Write};
mod task;
mod planner;
mod storage;

fn input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input_str = String::new();
    io::stdin().read_line(&mut input_str).unwrap();
    input_str.trim().to_string()
}

fn main() {
    let mut planner = planner::Planner::new();

    loop {
        let command = input("\nEnter command (add, list, complete, remove, edit, exit): ");

        match command.as_str() {
            "add" => {
                let description = input("Enter task description: ");
                if description.is_empty() {
                    println!("Task description cannot be empty.");
                } else {
                    planner.add_task(description);
                    planner.save().unwrap();
                    println!("Task added.");
                }
            }
            "list" => {
                planner.list_tasks();
            }
            "complete" => {
                let id = input("Enter task ID to complete: ").parse::<u32>().unwrap_or(0);
                if planner.mark_task_complete(id) {
                    planner.save().unwrap();
                    println!("Task marked complete.");
                } else {
                    println!("Task not found.");
                }
            }
            "remove" => {
                let id = input("Enter task ID to remove: ").parse::<u32>().unwrap_or(0);
                if planner.remove_task(id) {
                    planner.save().unwrap();
                    println!("Task removed.");
                } else {
                    println!("Task not found.");
                }
            }
            "edit" => {
                let id = input("Enter task ID to edit: ").parse::<u32>().unwrap_or(0);
                if id == 0 {
                    println!("Invalid ID.");
                    continue;
                }
                let new_description = input("Enter new task description: ");
                if new_description.is_empty() {
                    println!("Description cannot be empty.");
                    continue;
                }
                if planner.edit_task_description(id, new_description) {
                    planner.save().unwrap();
                    println!("Task description updated.");
                } else {
                    println!("Task not found.");
                }
            }
            "exit" => {
                println!("Exiting WeekPlanner.");
                break;
            }
            _ => println!("Unknown command."),
        }
    }
}
