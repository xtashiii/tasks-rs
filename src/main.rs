use rusqlite::{Connection, Result};
use std::io;

struct Task {
    id: i32,
    name: String,
    completed: bool,
}

impl Task {
    fn new(name: String) -> Task {
        Task {
            id: 0,
            name,
            completed: false,
        }
    }
}

fn main() -> Result<()>
{
    let mut tasks: Vec<Task> = Vec::new();
    let connection = Connection::open("tasks.db")?;
    
    connection.execute(
        "CREATE TABLE IF NOT EXISTS task (
            id INTEGER,
            name TEXT NOT NULL,
            completed BOOL,
            PRIMARY KEY(id)
        )",
        ()
    )?;

    clear_terminal();

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
                clear_terminal();

                let mut task = String::new();
                
                println!("Type your task name: ");

                io::stdin()
                    .read_line(&mut task)
                    .expect("Error on input task");

                tasks.push(Task::new(task.clone()));
                println!("{}", task);
                connection.execute(
                    "INSERT INTO task (name, completed) VALUES (?1, ?2)",
                    (&task.trim(), false)
                )?;
                clear_terminal();
                println!("Task has been added succesfully\n");
                
            },
            "2" => {
                clear_terminal();
                println!("Tasks List:");
                show_tasks(&connection).unwrap();
            },
            "3" => {
                clear_terminal();
                println!("Tasks list:");
                show_tasks(&connection).unwrap();
                
                let mut task_index = String::new();
                
                println!("Type the task number to update");

                io::stdin()
                    .read_line(&mut task_index)
                    .expect("Error on input task index");

                println!("Type the new task name");

                let mut new_task_name = String::new();

                io::stdin()
                    .read_line(&mut new_task_name)
                    .expect("Error on input new task name");

                connection.execute(
                    "UPDATE task SET name = (?1) WHERE id = (?2)",
                    (new_task_name.trim(), task_index.trim())
                )?;

                clear_terminal();
                println!("Task name has been updated succesfully\n");
             
            },
            "4" => {
                clear_terminal();
                println!("Tasks list:");
                show_tasks(&connection).unwrap();

                let mut task_index = String::new();
                
                println!("Type the task id to mark as completed");

                io::stdin()
                    .read_line(&mut task_index)
                    .expect("Error on input task index");

                connection.execute(
                    "UPDATE task SET completed = (?1) WHERE id = (?2)",
                    (true, task_index.trim())
                )?;
            },
            "5" => {
                clear_terminal();
                connection.execute(
                    "DELETE FROM task WHERE completed = ?1",
                    (true,)
                )?;
                println!("Completed tasks was deleted succesfully\n");
            }
            "6" => break,
            _ => {
                clear_terminal();
                println!("Invalid option.\n");                
            }
        }
    }
    clear_terminal();
    
    Ok(())
}

fn clear_terminal()
{
    print!("\x1B[2J\x1B[1;1H");
}

fn show_tasks(connection: &Connection) -> Result<()>
{
    let mut get_tasks = connection.prepare("SELECT * FROM task")?;
    let tasks = get_tasks.query_map([], |row| {
        Ok(Task {
            id: row.get(0)?,
            name: row.get(1)?,
            completed: row.get(2)?,
        })
    })?;

    for task in tasks {
        match task {
            Ok(task) => { 
                if task.completed {
                    println!("{}. [✓] {}", task.id, task.name);
                }
                else {
                    println!("{}. [ ] {}", task.id, task.name);
                }
            }
            Err(e) => println!("Error: {}", e)
        }
    }
    println!("");
    Ok(())
}