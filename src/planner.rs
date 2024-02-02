use crate::task::Task;

pub struct Planner {
    pub tasks: Vec<Task>,
}

impl Planner {

    pub fn add_task(&mut self, description: String) {
        let id = (self.tasks.len() as u32) + 1;
        let task = Task::new(id, description);
        self.tasks.push(task);
    }
    pub fn new() -> Self {
        Planner {
            tasks: Vec::new(),
        }
    }
}
