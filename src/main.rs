use std::io::{self, Write};
mod task;
mod planner;

fn main() {
    let mut planner = planner::Planner::new();

    loop {
        println!("\nWeekPlanner Menu:");
        println!("1. Add Task");
        println!("2. List Tasks");
        println!("3. Exit");
        print!("Enter choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        match choice {
            "1" => {
                println!("Enter task description:");
                let mut desc = String::new();
                io::stdin().read_line(&mut desc).unwrap();
                let desc = desc.trim().to_string();
                if desc.is_empty() {
                    println!("Task description cannot be empty.");
                } else {
                    planner.add_task(desc);
                    println!("Task added.");
                }
            }
            "2" => {
                planner.list_tasks();
            }
            "3" => {
                println!("Exiting WeekPlanner.");
                break;
            }
            _ => println!("Invalid choice. Please try again."),
        }
    }
}
