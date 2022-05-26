use std::env;
use std::ffi::OsStr;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::BufReader;
use std::path;
use termion::{color, style};
use question::{Answer, Question};

pub fn create_list() {
    if !list_does_not_exist() {
        println!(
            "{}List files already exist here.",
            color::Fg(color::LightRed)
        );
    } else {
        File::create("tasks.todo").expect("Error encountered while creating tasks file");
        File::create("completed.todo").expect("Error encountered while creating tasks file");
        println!("{}List files created", color::Fg(color::LightGreen))
    }
}

pub fn list_tasks() {
    if no_list_message(){
        return
    }

    let dir = env::current_dir().unwrap();
    let dir_name = OsStr::to_string_lossy(dir.file_name().unwrap());

    let mut title = "todo list at ".to_string();
    title.push_str(&dir_name);
    println!("");
    print_title(title);
    println!("\n\t{}Tasks to complete{}", style::Bold, style::Reset);

    match print_tasks() {
        Err(e) => println!("{:?}", e),
        _ => (),
    }

    println!("\n\t{}Tasks recently completed{}", style::Bold, style::Reset);
    match print_completed() {
        Err(e) => println!("{:?}", e),
        _ => (),
    }
}

pub fn add_task(task: String) {
    if no_list_message(){
        return
    }

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("tasks.todo")
        .unwrap();
    if let Err(e) = writeln!(file, "{}", task) {
        eprintln!("Couldn't write task to list: {}", e);
    }
    println!("{}Task added: {}", color::Fg(color::LightGreen), task)
}

pub fn finish_task(tasknum: usize) {
    if no_list_message(){
        return
    }

    let file_in = File::open("tasks.todo").expect("Tasks file cannot be found");
    let content = BufReader::new(&file_in);
    let mut lines: Vec<String> = content.lines().collect::<Result<_, _>>().unwrap();

    let task = lines[tasknum - 1].clone();

    lines.remove(tasknum - 1);

    let mut file_out = File::create("tasks.todo").expect("Tasks file cannot be accessed");
    for line in lines {
        if let Err(e) = writeln!(file_out, "{}", line) {
            eprintln!("Couldn't add task to todo list: {}", e);
        }
    }

    let mut file_completed = OpenOptions::new()
        .write(true)
        .append(true)
        .open("completed.todo")
        .unwrap();
    if let Err(e) = writeln!(file_completed, "{}", task) {
        eprintln!("Couldn't add task to completed: {}", e);
    }
    println!("{}Task completed: {}", color::Fg(color::LightGreen), task);
}


pub fn delete_list() -> std::io::Result<()> {
    if no_list_message(){
        return Ok(()); 
    }
    println!("This will irreversibly delete the todo list in this directory.");
    let answer = Question::new("Continue?")
        .default(Answer::NO)
        .show_defaults()
        .confirm();

    if answer == Answer::YES {
        fs::remove_file("tasks.todo")?;
        fs::remove_file("completed.todo")?;
        println!("{}Todo list deleted.", color::Fg(color::LightRed));
        Ok(())
    } else {
        println!("Aborted deletion.");
        Ok(())
    }
}


// Utils =================================================================

fn no_list_message() -> bool {
    if list_does_not_exist(){
        println!("{}A list does not exist here, use the create command.", color::Fg(color::LightRed));
        return true;
    }
    return false;
}

fn list_does_not_exist() -> bool {
    let mut dir: path::PathBuf = env::current_dir().unwrap();
    dir.push("tasks.todo");
    return !path::Path::new(&dir).exists();
}

fn print_title(text: String) {
    let (x, _y) = termion::terminal_size().unwrap();
    let width = (x as usize - text.len()) / 2;
    let mut text_centred = (0..width).map(|_| " ").collect::<String>();
    text_centred.push_str(&text);
    println!("{}{}{}", style::Bold, text_centred, style::Reset);
}

fn print_tasks() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("tasks.todo")?;
    let reader = BufReader::new(file);

    for (i, line) in reader.lines().enumerate() {
        println!("{}\t{} {}{}", color::Fg(color::LightRed), i + 1, line?, color::Fg(color::Reset));
    }

    Ok(())
}

fn print_completed() -> Result<(), Box<dyn std::error::Error>> {
    let file_in = File::open("completed.todo").expect("Completed tasks file cannot be found");
    let content = BufReader::new(&file_in);
    let mut lines: Vec<String> = content.lines().collect::<Result<_, _>>().unwrap();
    if lines.len() > 0 {

        lines.reverse();
        let mut recent_tasks = &lines[0..];
        if lines.len() > 5 {
            recent_tasks = &lines[0..5];
        } 

        for (i, line) in recent_tasks.iter().enumerate() {
            println!("{}\t{} {}{}", color::Fg(color::LightGreen), lines.len()-(i), line, color::Fg(color::Reset));
        }
        
    }
    Ok(())
}