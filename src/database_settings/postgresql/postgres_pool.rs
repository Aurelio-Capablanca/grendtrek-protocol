use crate::database_settings::connections::{DATABASE_REGISTRY, DatabaseConnections};
use axum::http::Response;
use std::hash::Hash;
//use crate::models::data_schema::DataSchema;

pub async fn make_a_simple_query(
    connection_name: &String,
) -> Result<(), Box<dyn std::error::Error>> {
    let connection_getter = DATABASE_REGISTRY
        .get_connection_pool(connection_name)
        .unwrap();
    let mut get = connection_getter.lock().unwrap();
    match &mut *get {
        DatabaseConnections::Postgres(pool) => {
            let rows : Vec<(String, String, String)> = sqlx::query_as("select table_catalog, table_schema, table_name from information_schema.\"tables\" t ")
                .fetch_all(&*pool)
                .await?;
            for (table_catalog, table_schema, table_name) in rows {
                println!(
                    "table catalog: {}; table schema: {}, table name: {}",
                    table_catalog, table_schema, table_name
                )
            }
        }
        _ => {
            println!("no Actions here! you meeant to use PostgreSQL connection!")
        }
    }
    Ok(())
}

pub async fn create_schemas(
    connection_name: &String,
    schemas: Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let connection = DATABASE_REGISTRY
        .get_connection_pool(connection_name)
        .unwrap();
    let mut getter = connection.lock().unwrap();
    match &mut *getter {
        DatabaseConnections::Postgres(pool) => {
            let mut transaction = pool.begin().await?;
            let mut success = true;
            for schema in schemas {
                let mut query = "create schema ".to_string();
                query.push_str(schema.as_str());
                let results = sqlx::query(query.as_str()).execute(&mut *transaction).await;
                match &results {
                    Ok(_) => success = true,
                    Err(e) => {
                        println!("Error {}", e);
                        success = false;
                    }
                }
                println!("result : {:?}", results)
                // pool
            }
            if success {
                transaction.commit().await?;
            } else {
                transaction.rollback().await?;
            }
        }
        _ => println!("You're supposed to use a PostgreSQL connection, please retry!!!"),
    }
    Ok(())
}
