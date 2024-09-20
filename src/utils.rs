use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: i32,
    pub name: String,
    pub completed: bool,
}

impl Task {
    pub fn new(name: String) -> Task {
        Task {
            id: 0,
            name,
            completed: false,
        }
    }
}

#[derive(serde::Deserialize)]
pub struct UpdateTaskData {
    pub name: Option<String>,
    pub completed: Option<bool>,
}

pub fn clear_terminal()
{
    print!("\x1B[2J\x1B[1;1H");
}