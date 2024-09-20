use rusqlite::Connection;

pub  fn start_db() -> Connection {
    let connection: Connection = Connection::open("tasks.db").unwrap();
    
    connection.execute(
        "CREATE TABLE IF NOT EXISTS task (
            id INTEGER,
            name TEXT NOT NULL,
            completed BOOL,
            PRIMARY KEY(id)
        )",
        ()
    ).unwrap();
    connection
}