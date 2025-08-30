use tiberius::AuthMethod;
use crate::common::grend_trek_error::StopTrek;
use crate::database_settings::connections;
use crate::database_settings::connections::{DatabaseRegistry, DATABASE_REGISTRY};
use crate::database_settings::postgresql::postgres_pool;
use crate::database_settings::sql_server::sql_server_pool;

mod common;
mod controllers;
mod database_settings;
mod models;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let postgres_url = "postgres://superuserp:jkl555@localhost:5432/transcontinentalshippings";
    DATABASE_REGISTRY.add_postgres_connection("PostgresSQLDestiny", postgres_url).await?;
    
    let mut configuration = tiberius::Config::new();
    configuration.host("localhost");
    configuration.port(1433);
    configuration.authentication(AuthMethod::sql_server("sa","jklgHnbvc555SS"));
    configuration.database("AdventureWorks2022");
    configuration.trust_cert();
    DATABASE_REGISTRY.add_mssql_connection("SQLServerADWorks", configuration).await?;


    DATABASE_REGISTRY.test_connection("SQLServerADWorks").await?;
    DATABASE_REGISTRY.test_connection("PostgresSQLDestiny").await?;


   postgres_pool::make_a_simple_query(&"PostgresSQLDestiny".to_string()).await?;
    let schemas = sql_server_pool::get_all_schemas(&"SQLServerADWorks".to_string()).await;
    let mut list_schemas: Vec<String> = Vec::new();
    match schemas {
        Ok(value) => {
            println!("no error");
            list_schemas.extend(value);
        }
        Err(e) => { 
            eprintln!("Error Got: {:?}",e)
        }
    }
    list_schemas.iter().for_each(|x| println!("{:?}",x));

    postgres_pool::create_schemas(&"PostgresSQLDestiny".to_string(), list_schemas).await?;
    Ok(())
}
