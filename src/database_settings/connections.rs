use sqlx::{PgPool, Row};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tiberius::Client;
use tokio::net::TcpStream;
use tokio_util::compat::TokioAsyncReadCompatExt;

pub enum DatabaseConnections {
    Postgres(PgPool),
    SQLServer(Client<tokio_util::compat::Compat<TcpStream>>),
}

pub struct DatabaseRegistry {
    connections: Arc<Mutex<HashMap<String, DatabaseConnections>>>,
}

impl DatabaseRegistry {
    pub fn new() -> Self {
        Self {
            connections: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn add_postgres_connection(
        &mut self,
        name: &str,
        url: &str,
    ) -> Result<(), sqlx::Error> {
        let pool = PgPool::connect(url).await?;
        let mut connections = self.connections.lock().unwrap();
        connections.insert(name.to_string(), DatabaseConnections::Postgres(pool));
        Ok(())
    }

    pub async fn add_mssql_connection(
        &mut self,
        name: &str,
        config: tiberius::Config,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let tcp = TcpStream::connect(config.get_addr()).await?;
        let getter = tcp.compat();
        let client = Client::connect(config, getter).await?;
        let mut connections = self.connections.lock().unwrap();
        connections.insert(name.to_string(), DatabaseConnections::SQLServer(client));
        Ok(())
    }

    pub async fn test_connection(&mut self, name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut connections = self.connections.lock().unwrap();
        match connections.get_mut(name) {
            Some(DatabaseConnections::Postgres(pool)) => {
                let result = sqlx::query("SELECT 1 as result").fetch_one(&*pool).await;
                match result {
                    Ok(row) => {
                        let value: i32 = row.try_get("result")?;
                        print!("PostgreSQL Connection: '{}'", value);
                    }
                    Err(err) => {
                        print!(
                            "Error at execution '{}': query or connection failed: {}",
                            name, err
                        );
                        return Err(Box::new(err));
                    }
                }
                Ok(())
            }
            Some(DatabaseConnections::SQLServer(client)) => {
                let rows = client
                    .query("SELECT 1 AS result", &[])
                    .await?
                    .into_first_result()
                    .await?;
                for row in rows {
                    let result: i32 = row.get("result").unwrap();
                    print!("Value: '{}'", result)
                }
                Ok(())
            }
            None => Err(format!("No connector added! '{}'", name).into()),
        }
    }
}
