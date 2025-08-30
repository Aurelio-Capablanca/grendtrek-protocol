use crate::common::grend_trek_error::StopTrek;
use crate::database_settings::connections::{DATABASE_REGISTRY, DatabaseConnections};
pub async fn get_all_schemas(
    connection_name: &String,
) -> Result<Vec<String>, Box<StopTrek>> {
    let connection = DATABASE_REGISTRY
        .get_connection_pool(connection_name)
        .unwrap();
    let mut getter = connection.lock().unwrap();
    let mut schemas: Vec<String> = Vec::new();
    match &mut *getter {
        DatabaseConnections::SQLServer(client) => {
            let query = client
                .query("SELECT SCHEMA_NAME from INFORMATION_SCHEMA.SCHEMATA s where s.SCHEMA_OWNER = 'dbo'", &[])
                .await
                .map_err(|e| {
                    eprintln!("Query failed: {e}");
                    StopTrek::Tiberius(e)
                })?
                .into_first_result()
                .await
                .map_err(|e| {
                    eprintln!("Failed to get first result: {e}");
                    StopTrek::Tiberius(e)
                })?;

            for row in query {
                let name: &str = row.get("SCHEMA_NAME").unwrap();
                schemas.push(name.to_string());
            }
        }
        _ => println!(
            "You are aiming to use a SQL Server Connection, please retry and verify your connection list!"
        ),
    }
    Ok(schemas)
}

pub async fn get_table_info_by_schema(
    connection_name: &String,
) -> Result<(), Box<StopTrek>> {

    Ok(())
}
