use tiberius::AuthMethod;
use crate::database_settings::connections::DatabaseRegistry;

mod common;
mod controllers;
mod database_settings;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut registry = DatabaseRegistry::new();
    
    let postgres_url = "postgres://superuserp:jkl555@localhost:5432/transcontinentalshippings"; 
    registry.add_postgres_connection("PostgresSQLDestiny", postgres_url).await?;
    
    let mut configuration = tiberius::Config::new();
    configuration.host("localhost");
    configuration.port(1433);
    configuration.authentication(AuthMethod::sql_server("sa","jklgHnbvc555SS"));
    configuration.database("AdventureWorks2022");
    configuration.trust_cert();
    registry.add_mssql_connection("SQLServerADWorks", configuration).await?;
    registry.test_connection("SQLServerADWorks").await?;
    registry.test_connection("PostgresSQLDestiny").await?;
    Ok(())
}
