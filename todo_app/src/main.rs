#[derive(Debug)]
struct Task {
    description: String,
    done: bool,
}

fn main() {
    let mut tasks: Vec<Task> = Vec::new();
    loop {
        print!(
            "
Choose an option
1. Add task
2. List tasks
3. Mark task as done
4. Exit
"
        );
        let mut user_option_raw = String::new();
        std::io::stdin()
            .read_line(&mut user_option_raw)
            .expect("User input error");
        let user_option: u32 = match user_option_raw.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Couldn't parse option");
                continue;
            }
        };

        if user_option > 4 {
            println!("Outside options boundaries, select one from the list");
            continue;
        } else {
            if user_option == 1 {
                println!("Enter task description:");
                let mut user_task_description = String::new();
                std::io::stdin()
                    .read_line(&mut user_task_description)
                    .expect("User input error");

                tasks.push(Task {
                    description: user_task_description.trim().to_string(),
                    done: false,
                });
                println!("Task added!");
            } else if user_option == 3 {
                println!("Enter task number");

                let mut task_number = String::new();
                std::io::stdin()
                    .read_line(&mut task_number)
                    .expect("User input error");
                let task_number: u32 = match task_number.trim().parse() {
                    Ok(n) => n,
                    Err(_) => {
                        println!("Error input, please try again");
                        continue;
                    }
                };
                tasks[task_number as usize - 1 as usize].done = true;
                println!("Task marked as done!");
            } else if user_option == 2 {
                for (i, task) in tasks.iter().enumerate() {
                    if task.done == true {
                        println!("{i}. [x] {0}", task.description)
                    }
                    if task.done == false {
                        println!("{i}. [] {0}", task.description)
                    }
                }
            } else if user_option == 4 {
                println!("Goodbye!");
                break;
            }
        }
    }
}
