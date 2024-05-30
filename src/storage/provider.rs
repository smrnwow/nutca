use rusqlite::Connection;

#[derive(Debug)]
pub struct Provider {
    connection: Connection,
}

impl Provider {
    pub fn new() -> Self {
        Self {
            connection: Connection::open_in_memory().unwrap(),
        }
    }

    pub fn connection(&self) -> &Connection {
        &self.connection
    }
}
