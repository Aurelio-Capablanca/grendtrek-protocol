use crate::database_settings::connections::{DatabaseConnections, DATABASE_REGISTRY};
//use crate::models::data_schema::DataSchema;

pub async fn make_a_simple_query(connection_name: &String) -> Result<(), Box<dyn std::error::Error>>{
    let connection_getter = DATABASE_REGISTRY.get_connection_pool(connection_name).unwrap();
    let mut get = connection_getter.lock().unwrap();
    match &mut *get {
        DatabaseConnections::Postgres(pool)=> {
            let rows : Vec<(String, String, String)> = sqlx::query_as("select table_catalog, table_schema, table_name from information_schema.\"tables\" t ")
                .fetch_all(&*pool)
                .await?;
            for (table_catalog,table_schema,table_name ) in rows {
                println!("table catalog: {}; table schema: {}, table name: {}",table_catalog,table_schema,table_name)
            }
        },
        _ => {
            println!("no Actions here! you meeant to use PostgreSQL connection!")
        }
    }
    Ok(())
}


//pub async fn 