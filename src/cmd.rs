use std::env;
use std::fs::File;
use std::path;
use termion::color;

pub fn create_list() {
    if check_list_exists() {
        println!(
            "{}List files already exist here.",
            color::Fg(color::LightRed)
        );
    } else {
        File::create("tasks.todo").expect("Error encountered while creating tasks file");
        File::create("completed.todo").expect("Error encountered while creating tasks file");
        println!("List files created.")
    }
}

pub fn list_tasks() {
    println!("called list command");
}

pub fn add_task(task: String) {
    println!("called add task command with {}", task);
}

pub fn finish_task(tasknum: i32) {
    println!("called finish task command on task {}", tasknum);
}

pub fn delete_list() {
    println!("called delete list command");
}

fn check_list_exists() -> bool {
    let mut dir: path::PathBuf = env::current_dir().unwrap();
    dir.push("tasks.todo");
    return path::Path::new(&dir).exists();
}
