use std::env;
use std::ffi::OsStr;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::BufReader;
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
    if check_list_exists() {
        let dir = env::current_dir().unwrap();
        let dir_name = OsStr::to_string_lossy(dir.file_name().unwrap());

        let mut title = "List at ".to_string();
        title.push_str(&dir_name);
        print_centred(title);

        match print_file("tasks.todo") {
            Err(e) => println!("{:?}", e),
            _ => (),
        }
    } else {
        println!("A list does not exist here");
    }
}

pub fn add_task(task: String) {
    if check_list_exists() {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open("tasks.todo")
            .unwrap();
        if let Err(e) = writeln!(file, "{}", task) {
            eprintln!("Couldn't write task to list: {}", e);
        }
        println!("{}Task added: {}", color::Fg(color::LightGreen), task)
    } else {
        println!("A list does not exist here")
    }
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

fn print_centred(text: String) {
    let (x, _y) = termion::terminal_size().unwrap();
    let width = (x as usize - text.len()) / 2;
    let mut text_centred = (0..width).map(|_| " ").collect::<String>();
    text_centred.push_str(&text);
    println!("{}", text_centred);
}

fn print_file(filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    for (i, line) in reader.lines().enumerate() {
        println!("{}\t{} {}", color::Fg(color::LightRed), i + 1, line?);
    }

    Ok(())
}
