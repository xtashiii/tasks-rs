use std::io;

struct Task {
    name: String,
    completed: bool,
}

impl Task {
    fn new(name: String) -> Task {
        Task {
            name,
            completed: false,
        }
    }

    fn mark_completed(&mut self) {
        self.completed = true;
    }
}

fn main()
{
    let mut tasks: Vec<Task> = Vec::new();
    print!("\x1B[2J\x1B[1;1H");

    loop {
        println!("Menu Task List Mannager:");
        println!("1. Add task");
        println!("2. See tasks");
        println!("3. Update task");
        println!("4. Complete task");
        println!("5. Delete completed tasks");
        println!("6. Exit");
        
        let mut buffer = String::new();

        io::stdin().read_line(&mut buffer).expect("Error");

        match buffer.trim() {
            "1" => {
                print!("\x1B[2J\x1B[1;1H");

                let mut task = String::new();
                
                println!("Type your task name: ");

                io::stdin()
                    .read_line(&mut task)
                    .expect("Error on input task");

                tasks.push(Task::new(task));

                print!("\x1B[2J\x1B[1;1H");
                println!("Task has been added succesfully\n");
                
            },
            "2" => {
                print!("\x1B[2J\x1B[1;1H");
                println!("Tasks list:");

                let mut count = 1;

                for task in tasks.iter() {
                    if task.completed {
                        print!("{}. [✓] {}", count, task.name);
                    }
                    else {
                        print!("{}. [ ] {}", count, task.name);

                    }
                    
                    count += 1;
                }
                println!("");
            },
            "3" => {
                let mut task_index = String::new();
                let mut count = 1;

                print!("\x1B[2J\x1B[1;1H");
                println!("Tasks list:");

                for task in tasks.iter() {
                    if task.completed {
                        print!("{}. [✓] {}", count, task.name);
                    }
                    else {
                        print!("{}. [ ] {}", count, task.name);

                    }
                    
                    count += 1;
                }

                println!("Type the task number to update");

                io::stdin()
                    .read_line(&mut task_index)
                    .expect("Error on input task index");


                match task_index.trim().parse::<usize>() {
                    Ok(index) if index > 0 && index < tasks.len() + 1 =>
                    {
                        println!("Type the new task name");

                        let mut new_task_name = String::new();

                        io::stdin()
                            .read_line(&mut new_task_name)
                            .expect("Error on input new task name");
                        
                        tasks[index - 1].name = new_task_name;

                        print!("\x1B[2J\x1B[1;1H");
                        println!("Task name has been updated succesfully\n")
                    },
                    _ => {
                        print!("\x1B[2J\x1B[1;1H");
                        println!("Error on update task name\n");
                    },
                }
            },
            "4" => {
                let mut task_index = String::new();
                let mut count = 1;

                print!("\x1B[2J\x1B[1;1H");
                println!("Tasks list:");

                for task in tasks.iter() {
                    if task.completed {
                        print!("{}. [✓] {}", count, task.name);
                    }
                    else {
                        print!("{}. [ ] {}", count, task.name);

                    }
                    
                    count += 1;
                }

                println!("Type the task number to update\n");

                io::stdin()
                    .read_line(&mut task_index)
                    .expect("Error on input task index");


                match task_index.trim().parse::<usize>() {
                    Ok(index)  if index > 0 && index < tasks.len() + 1 
                        && !tasks[index - 1].completed => 
                        {
                            tasks[index - 1].mark_completed();

                            print!("\x1B[2J\x1B[1;1H");
                            println!("Task marked as completed\n")
                        },
                    _ => {
                        print!("\x1B[2J\x1B[1;1H");
                        println!("Error");
                    }
                }
            },
            "5" => {
                tasks.retain(|task| !task.completed); // retain not ✓ 

                print!("\x1B[2J\x1B[1;1H");
                println!("Completed tasks was deleted succesfully\n");
            }
            "6" => break,
            _ => {
                print!("\x1B[2J\x1B[1;1H");
                println!("Invalid option.\n");                
            }
        }
    }
    print!("\x1B[2J\x1B[1;1H");
}