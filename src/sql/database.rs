use sqlx::{
    any::{Any, AnyPoolOptions, AnyRow},
    Pool,
};

pub enum SqlThread {
    TableRow(AnyRow),
}

#[derive(Debug)]
pub struct DatabaseConnection {
    pub connection_string: String,
    pub pool: Option<Pool<Any>>,
}

impl DatabaseConnection {
    pub fn new(connection_string: String) -> Self {
        DatabaseConnection {
            connection_string,
            pool: None,
        }
    }

    pub fn set_pool(&mut self) -> Result<(), sqlx::Error> {
        self.pool = Some(
            AnyPoolOptions::new()
                .max_connections(5)
                .connect_lazy(&self.connection_string)?,
        );

        Ok(())
    }
}

#[derive(Debug)]
pub struct DatabaseConnectionList {
    pub list: Vec<DatabaseConnection>,
}

impl DatabaseConnectionList {
    pub fn new() -> Self {
        DatabaseConnectionList { list: Vec::new() }
    }
}
