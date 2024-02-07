use crate::task::Task;
use serde::{Serialize, Deserialize};
use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, BufWriter};

const FILE_PATH: &str = "tasks.json";

pub fn save_tasks(tasks: &Vec<Task>) -> io::Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(FILE_PATH)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, tasks)?;
    Ok(())
}

pub fn load_tasks() -> io::Result<Vec<Task>> {
    let file = match File::open(FILE_PATH) {
        Ok(file) => file,
        Err(_) => return Ok(Vec::new()), // No file means no tasks yet
    };
    let reader = BufReader::new(file);
    let tasks = serde_json::from_reader(reader)?;
    Ok(tasks)
}
