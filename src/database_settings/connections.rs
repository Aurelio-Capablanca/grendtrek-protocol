use sqlx::PgPool;
use std::collections::HashMap;
use tiberius::Client;
use tokio::net::TcpStream;

pub enum DatabaseConnections {
    Postgres(PgPool),
    SQLServer(Client<tokio_util::compat::Compat<TcpStream>>),
}

pub struct DatabaseRegistry {
    connections: HashMap<String, DatabaseConnections>,
}

impl DatabaseRegistry {
    pub fn new() -> Self {
        Self {
            connections: HashMap::new(),
        }
    }
    
    pub async fn add_postgres_connection(&mut self, name :&str, url: &str) -> Result<(), sqlx::Error>{
        let pool = PgPool::connect(url).await?;
        self.connections.insert(name.to_string(), DatabaseConnections::Postgres(pool));
        Ok(())
    }
    
    
    
}
