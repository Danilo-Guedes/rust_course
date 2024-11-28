#[derive(Debug, Clone, PartialEq, Eq)]
enum Priority {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone)]
struct Task {
    name: String,
    priority: Priority,
}

impl Task {
    fn new(name: &str, priority: Priority) -> Self {
        Task {
            name: name.to_string(),
            priority,
        }
    }
}

struct PriorityIterator<'a> {
    tasks: &'a [Task],
    current_priority: Priority,
    index: usize,
}

impl<'a> PriorityIterator<'a> {
    fn new(tasks: &'a [Task], current_priority: Priority) -> Self {
        Self {
            tasks,
            current_priority,
            index: 0,
        }
    }
}

impl<'a> Iterator for PriorityIterator<'a> {
    type Item = &'a Task;
    fn next(&mut self) -> Option<Self::Item> {
        while self.index < self.tasks.len() {
            let curr_task = &self.tasks[self.index];
            self.index += 1;
            if self.current_priority == curr_task.priority {
                return Some(curr_task);
            } 
            // else {
            //     if self.current_priority == Priority::High {
            //         self.current_priority = Priority::Medium,
            //     } else if self.current_priority == Priority::Medium {
            //         self.current_priority = Priority::Low
            //     }
            // }
        }
        None
    }
}

fn execute_task(task: &Task) {
    println!("Executing {:?}: {:?}", task.name, task.priority);
}

fn main() {
    let all_tasks = vec![
        Task::new("Laundry", Priority::Low),
        Task::new("Emails", Priority::High),
        Task::new("Homework", Priority::Medium),
        Task::new("Cleaning", Priority::Medium),
        Task::new("Taxes", Priority::High),
    ];

    println!("Execute tasks by priority!");

    let prioritized_tasks = PriorityIterator::new(&all_tasks, Priority::High);

    for task in prioritized_tasks {
        execute_task(task);
    }
}
