use once_cell::sync::Lazy;
use sqlx::{PgPool, Row};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::option::Option;
use tiberius::Client;
use tokio::net::TcpStream;
use tokio_util::compat::TokioAsyncReadCompatExt;
use crate::common::grend_trek_error::StopTrek;

pub enum DatabaseConnections {
    Postgres(PgPool),
    SQLServer(Client<tokio_util::compat::Compat<TcpStream>>),
    None
}

pub struct DatabaseRegistry {
    connections: Arc<Mutex<HashMap<String, Arc<Mutex<DatabaseConnections>>>>>,
}

pub static DATABASE_REGISTRY: Lazy<DatabaseRegistry> = Lazy::new(|| DatabaseRegistry::new());

impl DatabaseRegistry {
    pub fn new() -> Self {
        Self {
            connections: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn add_postgres_connection(&self, name: &str, url: &str) -> Result<(), sqlx::Error> {
        let pool = PgPool::connect(url).await?;
        let mut connections = self.connections.lock().unwrap();
        connections.insert(
            name.to_string(),
            Arc::new(Mutex::new(DatabaseConnections::Postgres(pool))),
        );
        Ok(())
    }

    pub async fn add_mssql_connection(
        &self,
        name: &str,
        config: tiberius::Config,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let tcp = TcpStream::connect(config.get_addr()).await?;
        let getter = tcp.compat();
        let client = Client::connect(config, getter).await?;
        let mut connections = self.connections.lock().unwrap();
        connections.insert(
            name.to_string(),
            Arc::new(Mutex::new(DatabaseConnections::SQLServer(client))),
        );
        Ok(())
    }

    pub fn get_connection_pool(&self, name: &str) -> Option<Arc<Mutex<DatabaseConnections>>> {
        let connections = self.connections.lock().unwrap();
        connections.get(name).cloned()
    }

    pub async fn test_connection(&self, name: &str) -> Result<(),StopTrek> {
        let mut connections = self.connections.lock().unwrap();
        match connections.get_mut(name) {
            Some(con) => {
                let mut lock_connections = con.lock().unwrap();
                match &mut *lock_connections {
                    DatabaseConnections::Postgres(pool) => {
                        let result = sqlx::query("SELECT 1 as result").fetch_one(&*pool).await;
                        match result {
                            Ok(row) => {
                                let value: i32 = row.try_get("result").map_err(|e| {StopTrek::SQLx(e)})?;
                                print!("PostgreSQL Connection: '{}'", value);
                            }
                            Err(err) => {
                                print!(
                                    "Error at execution '{}': query or connection failed: {}",
                                    name, err
                                );
                                return Err(StopTrek::SQLx(err));
                            }
                        }
                        Ok(())
                    }
                    DatabaseConnections::SQLServer(client) => {
                        let rows = client
                            .query("SELECT 1 AS result", &[])
                            .await
                            .map_err(|e1| {StopTrek::Tiberius(e1)})?
                            .into_first_result()
                            .await.map_err(|e2| {StopTrek::Tiberius(e2)})?;
                        for row in rows {
                            let result: i32 = row.get("result").unwrap();
                            print!("SQL Server Connection: '{}'", result)
                        }
                        Ok(())
                    }
                    _=> {
                        Err(StopTrek::CustomMessage("No Action".to_string()))
                    }
                }
            }
            None => Err(StopTrek::CustomMessage(format!("No connector is Used! '{}'", name))),
        }
    }
}
