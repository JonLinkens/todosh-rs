pub fn create_list() {
    println!("called create list command");
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
