use serde::{Deserialize, Serialize};
use serde_yaml;

#[derive(Debug, Serialize, Deserialize)]
struct task {
    name: String,
    done: bool,
    tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TaskList {
    tasks: Vec<task>,
}

fn main() {
    
}
