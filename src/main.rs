use std::io;

// add task function
fn add_task(taskslist: &mut Vec<String>) {
    loop {
            println!("insert a task");
            let mut new_task = String::new();
            io::stdin()
                .read_line(&mut new_task)
                .expect("failed to new task");
            taskslist.push(new_task.trim().to_string());
            println!("success add new task");
            println!("want to add new task? [y/n]");
            let mut confirmation = String::new();
            io::stdin()
                .read_line(&mut confirmation)
                .expect("can't read confirmation");
            let confirmation =confirmation.trim();
            match confirmation {
                "y" => continue,
                "n" => break,
                _ => println!("enter [y/n]!"),
            }
    }
}
// show task function
fn show_tasks(taskslist: &mut Vec<String>) {
    println!("Your Tasks: ");
    for my_task in taskslist {
        println!("{}",my_task)
    }
}
fn remove_task(taskslist: &mut Vec<String>) {
    println!("enter the task you want to remove: ");
    let mut task_remove = String::new();
    io::stdin()
        .read_line(&mut task_remove)
        .expect("failed to get task remove");
    let task_remove = task_remove.trim();
    // find position of the input then remove it use the index -- look more at opt<T>
    if let Some(index) = taskslist.iter().position(|item|item.trim() == task_remove) {
        taskslist.remove(index);
    }
}
fn main() {
    // show welcome message
    println!("hello, what will you do today?");
    // loop with options: add, list task, remove task, exit => read_line (), String::new(), trim().parse()
    // store task in memory (use vector) vec<String> it should be outside the loop so it wouldn't create empty everytime
    let mut tasks: Vec<String>=Vec::new();
    loop {
        let options = ["add", "remove", "show tasks", "exit"];
        println!("choose one option");
        for opt in &options {
            println!("{}",opt);
        }

        let mut my_command= String::new();
        io::stdin()
            .read_line(&mut my_command)
            .expect("failed to readline");
        let my_command = my_command.trim();
        

        match my_command {
            "add" => add_task(&mut tasks),
            "remove" => remove_task(&mut tasks),
            "show tasks" => show_tasks(&mut tasks),
            "exit" => break,
            _=> println!("enter valid entry!")
        }
    }
}
