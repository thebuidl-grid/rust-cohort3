use std::io::{self, Write};

// ANSI color codes
const GREEN: &str = "\x1b[32m";
const BLUE: &str = "\x1b[34m";
const YELLOW: &str = "\x1b[33m";
const CYAN: &str = "\x1b[36m";
const RED: &str = "\x1b[31m";
const RESET: &str = "\x1b[0m";

// Task structure with id, title, description, optional reminder, and completion status
#[derive(Debug)]
struct Task {
    id: usize,
    title: String,
    description: String,
    reminder: Option<String>,
    done: bool,
}

// Manages collection of tasks
struct TaskManager {
    tasks: Vec<Task>,
    next_id: usize,
}


impl TaskManager {
    fn new() -> Self {
        Self { tasks: Vec::new(), next_id: 1 }
    }

    // Create new task
    fn create(&mut self, title: String, description: String, reminder: Option<String>) {
        self.tasks.push(Task { id: self.next_id, title, description, reminder, done: false });
        self.next_id += 1;
        println!("{GREEN}Task created!{RESET}");
    }

    // List all tasks
    fn read(&self) {
        if self.tasks.is_empty() {
            println!("{BLUE}No tasks.{RESET}");
            return;
        }
        for task in &self.tasks {
            println!("{BLUE}[{}] {} - {}{RESET}", task.id, if task.done { "✓" } else { " " }, task.title);
            println!("{BLUE}    {}{RESET}", task.description);
            if let Some(reminder) = &task.reminder {
                println!("{BLUE}    ⏰ {}{RESET}", reminder);
            }
        }
    }

    // Update existing task
    fn update(&mut self, id: usize, title: String, description: String, reminder: Option<String>) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.title = title;
            task.description = description;
            task.reminder = reminder;
            println!("{YELLOW}Task updated!{RESET}");
        } else {
            println!("{YELLOW}Task not found.{RESET}");
        }
    }

    // Toggle task completion status
    fn toggle(&mut self, id: usize) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.done = !task.done;
            println!("{CYAN}Task toggled!{RESET}");
        } else {
            println!("{CYAN}Task not found.{RESET}");
        }
    }

    // Delete task by id
    fn delete(&mut self, id: usize) {
        if let Some(pos) = self.tasks.iter().position(|t| t.id == id) {
            self.tasks.remove(pos);
            println!("{RED}Task deleted!{RESET}");
        } else {
            println!("{RED}Task not found.{RESET}");
        }
    }
}

fn main() {
    let mut manager = TaskManager::new();
    let stdin = io::stdin();

    loop {
        print!("{GREEN}\n1.Create 2.Read 3.Update 4.Toggle 5.Delete 6.Quit\n> {RESET}");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        match input.trim() {
            "1" => {
                print!("{GREEN}Title: {RESET}");
                io::stdout().flush().unwrap();
                let mut title = String::new();
                stdin.read_line(&mut title).unwrap();
                print!("{GREEN}Description: {RESET}");
                io::stdout().flush().unwrap();
                let mut desc = String::new();
                stdin.read_line(&mut desc).unwrap();
                print!("{GREEN}Reminder (or press Enter to skip): {RESET}");
                io::stdout().flush().unwrap();
                let mut reminder = String::new();
                stdin.read_line(&mut reminder).unwrap();
                let reminder = if reminder.trim().is_empty() { None } else { Some(reminder.trim().to_string()) };
                manager.create(title.trim().to_string(), desc.trim().to_string(), reminder);
            }
            "2" => manager.read(),
            "3" => {
                print!("{YELLOW}ID: {RESET}");
                io::stdout().flush().unwrap();
                let mut id = String::new();
                stdin.read_line(&mut id).unwrap();
                print!("{YELLOW}New title: {RESET}");
                io::stdout().flush().unwrap();
                let mut title = String::new();
                stdin.read_line(&mut title).unwrap();
                print!("{YELLOW}New description: {RESET}");
                io::stdout().flush().unwrap();
                let mut desc = String::new();
                stdin.read_line(&mut desc).unwrap();
                print!("{YELLOW}Reminder (or press Enter to skip): {RESET}");
                io::stdout().flush().unwrap();
                let mut reminder = String::new();
                stdin.read_line(&mut reminder).unwrap();
                let reminder = if reminder.trim().is_empty() { None } else { Some(reminder.trim().to_string()) };
                if let Ok(id) = id.trim().parse() {
                    manager.update(id, title.trim().to_string(), desc.trim().to_string(), reminder);
                }
            }
            "4" => {
                print!("{CYAN}ID: {RESET}");
                io::stdout().flush().unwrap();
                let mut id = String::new();
                stdin.read_line(&mut id).unwrap();
                if let Ok(id) = id.trim().parse() {
                    manager.toggle(id);
                }
            }
            "5" => {
                print!("{RED}ID: {RESET}");
                io::stdout().flush().unwrap();
                let mut id = String::new();
                stdin.read_line(&mut id).unwrap();
                if let Ok(id) = id.trim().parse() {
                    manager.delete(id);
                }
            }
            "6" => break,
            _ => println!("{GREEN}Invalid option.{RESET}"),
        }
    }
}
