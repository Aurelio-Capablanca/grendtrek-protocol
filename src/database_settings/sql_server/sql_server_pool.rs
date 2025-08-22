use crate::database_settings::connections::{DATABASE_REGISTRY, DatabaseConnections};
use tiberius::QueryStream;

pub async fn get_all_schemas(
    connection_name: &String,
) -> Result<(Vec<String>), Box<dyn std::error::Error>> {
    let connection = DATABASE_REGISTRY
        .get_connection_pool(connection_name)
        .unwrap();
    let mut getter = connection.lock().unwrap();
    let mut schemas:Vec<String> = Vec::new();
    match &mut *getter {
        DatabaseConnections::SQLServer(client) => {
            let query = client
                .query("SELECT SCHEMA_NAME from INFORMATION_SCHEMA.SCHEMATA s where s.SCHEMA_OWNER = 'dbo'", &[])
                .await
                .map_err(|e| {
                    eprintln!("Query failed: {e}");
                    e
                })?
                .into_first_result()
                .await
                .map_err(|e| {
                    eprintln!("Failed to get first result: {e}");
                    e
                })?;

            for row in query {
                let name:&str = row.get("SCHEMA_NAME").unwrap();
                schemas.push(name.to_string());
            }
        }
        _ => println!(
            "You are aiming to use a SQL Server Connection, please retry and verify your connection list!"
        ),
    }
    Ok(schemas)
}
