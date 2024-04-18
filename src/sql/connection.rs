use sqlx::{
    any::{Any, AnyPoolOptions, AnyRow},
    Pool,
};

pub enum SqlThread {
    TableRow(AnyRow),
}

#[derive(Debug)]
pub struct Connection {
    pub connection_string: String,
    pub pool: Option<Pool<Any>>,
}

impl Connection {
    pub fn new(connection_string: String) -> Self {
        Connection {
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
pub struct ConnectionList {
    pub list: Vec<Connection>,
}

impl ConnectionList {
    pub fn new() -> Self {
        ConnectionList { list: Vec::new() }
    }
}
