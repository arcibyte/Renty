use crate::task::Task;

pub struct Planner {
    pub tasks: Vec<Task>,
}

impl Planner {
    pub fn new() -> Self {
        Planner {
            tasks: Vec::new(),
        }
    }

    pub fn add_task(&mut self, description: String) {
        let id = (self.tasks.len() as u32) + 1;
        let task = Task::new(id, description);
        self.tasks.push(task);
    }

    pub fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("No tasks available.");
            return;
        }
        for task in &self.tasks {
            println!(
                "[{}] {} - {}",
                task.id,
                if task.completed { "x" } else { " " },
                task.description
            );
        }
    }

    pub fn mark_task_complete(&mut self, id: u32) -> bool {
        for task in &mut self.tasks {
            if task.id == id {
                task.completed = true;
                return true;
            }
        }
        false
    }
    pub fn remove_task(&mut self, id: u32) -> bool {
    if let Some(pos) = self.tasks.iter().position(|t| t.id == id) {
        self.tasks.remove(pos);
        true
    } else {
        false
    }
}

}
