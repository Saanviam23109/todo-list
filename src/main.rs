use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use colored::*;
use std::cmp::Ordering; // Import Ordering for implementing Ord and PartialOrd

// Define the Priority enum
#[derive(Debug, Clone)]
enum Priority {
    Low,
    Medium,
    High,
}

impl Priority {
    fn get_priority_str(&self) -> &str {
        match self {
            Priority::Low => "Low",
            Priority::Medium => "Medium",
            Priority::High => "High",
        }
    }
}

// Implementing Ord, PartialOrd, Eq, and PartialEq for the Priority enum
impl Ord for Priority {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Priority::Low, Priority::Low) => Ordering::Equal,
            (Priority::Low, _) => Ordering::Less,
            (_, Priority::Low) => Ordering::Greater,
            (Priority::Medium, Priority::Medium) => Ordering::Equal,
            (Priority::Medium, _) => Ordering::Less,
            (_, Priority::Medium) => Ordering::Greater,
            (Priority::High, Priority::High) => Ordering::Equal,
        }
    }
}

impl PartialOrd for Priority {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Priority {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Priority {}

// Define Task struct
#[derive(Debug)]
struct Task {
    description: String,
    completed: bool,
    priority: Priority,
}

impl Task {
    fn new(description: String, priority: Priority) -> Task {
        Task {
            description,
            completed: false,
            priority,
        }
    }

    fn mark_completed(&mut self) {
        self.completed = true;
    }

    fn get_priority_str(&self) -> &str {
        self.priority.get_priority_str()
    }
}

fn main() {
    let mut tasks: Vec<Task> = load_tasks(); // Load tasks from file
    loop {
        println!("\nTo-Do List:");
        println!("1. Add Task");
        println!("2. List Tasks");
        println!("3. Mark Task Completed");
        println!("4. Delete Task");
        println!("5. Sort Tasks");
        println!("6. Exit");

        let choice = get_input("Choose an option: ");
        match choice.trim() {
            "1" => add_task(&mut tasks),
            "2" => list_tasks(&tasks),
            "3" => mark_task_completed(&mut tasks),
            "4" => delete_task(&mut tasks),
            "5" => sort_tasks(&mut tasks),
            "6" => {
                save_tasks(&tasks);
                break;
            },
            _ => println!("Invalid choice, please try again."),
        }
    }
}

// Function to get user input
fn get_input(prompt: &str) -> String {
    let mut input = String::new();
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

// Add task to the list
fn add_task(tasks: &mut Vec<Task>) {
    let description = get_input("Enter task description: ");
    println!("Choose priority (1: Low, 2: Medium, 3: High): ");
    let priority_input = get_input("");
    let priority = match priority_input.trim() {
        "1" => Priority::Low,
        "2" => Priority::Medium,
        "3" => Priority::High,
        _ => {
            println!("Invalid priority. Defaulting to Low.");
            Priority::Low
        }
    };
    let task = Task::new(description, priority);
    tasks.push(task);
}

// List tasks with status and priority
fn list_tasks(tasks: &Vec<Task>) {
    if tasks.is_empty() {
        println!("No tasks available.");
    } else {
        for (index, task) in tasks.iter().enumerate() {
            let status = if task.completed { 
                task.description.green() 
            } else { 
                task.description.red() 
            };
            let priority = match task.priority {
                Priority::Low => "Low".yellow(),
                Priority::Medium => "Medium".cyan(),
                Priority::High => "High".bright_red(),
            };

            let status_str = if task.completed { 
                "Completed".green() 
            } else { 
                "Pending".red() 
            };

            println!("{}. {} - {} - {}", index + 1, status, status_str, priority);
        }
    }
}

// Mark task as completed
fn mark_task_completed(tasks: &mut Vec<Task>) {
    let task_index = get_input("Enter task number to mark as completed: ");
    if let Ok(index) = task_index.parse::<usize>() {
        if index > 0 && index <= tasks.len() {
            tasks[index - 1].mark_completed();
            println!("Task marked as completed.");
        } else {
            println!("Invalid task number.");
        }
    }
}

// Delete a task
fn delete_task(tasks: &mut Vec<Task>) {
    let task_index = get_input("Enter task number to delete: ");
    if let Ok(index) = task_index.parse::<usize>() {
        if index > 0 && index <= tasks.len() {
            tasks.remove(index - 1);
            println!("Task deleted.");
        } else {
            println!("Invalid task number.");
        }
    }
}

// Sort tasks by priority
fn sort_tasks(tasks: &mut Vec<Task>) {
    tasks.sort_by(|a, b| a.priority.cmp(&b.priority));
    println!("Tasks sorted by priority.");
}

// Save tasks to file
fn save_tasks(tasks: &Vec<Task>) {
    let file_path = "tasks.txt";
    let mut file = File::create(file_path).unwrap();
    for task in tasks {
        let priority_str = task.get_priority_str();
        let status = if task.completed { "Completed" } else { "Pending" };
        writeln!(file, "{}|{}|{}", task.description, status, priority_str).unwrap();
    }
}

// Load tasks from file
fn load_tasks() -> Vec<Task> {
    let file_path = "tasks.txt";
    if !Path::new(file_path).exists() {
        return Vec::new();
    }

    let contents = fs::read_to_string(file_path).unwrap();
    let mut tasks = Vec::new();
    for line in contents.lines() {
        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() == 3 {
            let completed = parts[1] == "Completed";
            let priority = match parts[2] {
                "Low" => Priority::Low,
                "Medium" => Priority::Medium,
                "High" => Priority::High,
                _ => Priority::Low, // Default to Low if invalid
            };
            let task = Task {
                description: parts[0].to_string(),
                completed,
                priority,
            };
            tasks.push(task);
        }
    }
    tasks
}
