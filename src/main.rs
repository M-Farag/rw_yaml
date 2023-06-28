use std::fs;

use serde::{Deserialize, Serialize};
use serde_yaml;

#[derive(Debug, Serialize, Deserialize)]
struct task {
    name: String,

    #[serde(skip)]
    done: bool,
    tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TaskList {
    tasks: Vec<task>,
}

fn main() {
    // read yaml file
    let file = fs::File::open("tasks.yml").expect("Err: Reading the file");
    let tasks:TaskList = serde_yaml::from_reader(file).expect("Err: Serializing the yml file");

    for task in tasks.tasks {
        println!("{:#?}",task);
    }
}
