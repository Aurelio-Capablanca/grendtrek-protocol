use crate::common::grend_trek_error::StopTrek;
use crate::database_settings::connections::{DATABASE_REGISTRY, DatabaseConnections};

pub async fn get_all_schemas(connection_name: &String) -> Result<Vec<String>, Box<StopTrek>> {
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
    schema_name: &String,
) -> Result<(), Box<StopTrek>> {
    let connection = DATABASE_REGISTRY
        .get_connection_pool(connection_name)
        .unwrap();
    let mut getter = connection.lock().unwrap();
    match &mut *getter {
        DatabaseConnections::SQLServer(client) => {
            let query = client
                .query(
                    "SELECT
	IC.COLUMN_NAME as column_name,
	IC.Data_TYPE as data_type,
	IC.CHARACTER_MAXIMUM_LENGTH as length_field,
	CAST(EP.[Value] as Nvarchar) as ms_description,
	IKU.CONSTRAINT_NAME as constraint_name,
	ITC.CONSTRAINT_TYPE as constraint_type,
	IC.IS_NULLABLE as is_nullable,
	IC.TABLE_NAME as table_name,
	IC.TABLE_SCHEMA as table_schema,
	IC.NUMERIC_PRECISION as numeric_precision,
	IC.NUMERIC_SCALE as numeric_scale
FROM
	INFORMATION_SCHEMA.COLUMNS IC
INNER JOIN sys.columns sc ON
	OBJECT_ID(QUOTENAME(IC.TABLE_SCHEMA) + '.' + QUOTENAME(IC.TABLE_NAME)) = sc.[object_id]
	AND IC.COLUMN_NAME = sc.name
LEFT OUTER JOIN sys.extended_properties EP ON
	sc.[object_id] = EP.major_id
	AND sc.[column_id] = EP.minor_id
	AND EP.name = 'MS_Description'
	AND EP.class = 1
LEFT OUTER JOIN INFORMATION_SCHEMA.KEY_COLUMN_USAGE IKU ON
	IKU.COLUMN_NAME = IC.COLUMN_NAME
	and IKU.TABLE_NAME = IC.TABLE_NAME
	and IKU.TABLE_CATALOG = IC.TABLE_CATALOG
LEFT OUTER JOIN INFORMATION_SCHEMA.TABLE_CONSTRAINTS ITC ON
	ITC.TABLE_NAME = IKU.TABLE_NAME
	and ITC.CONSTRAINT_NAME = IKU.CONSTRAINT_NAME
INNER JOIN INFORMATION_SCHEMA.TABLES t ON
	IC.TABLE_NAME = t.TABLE_NAME
WHERE
	IC.TABLE_CATALOG = 'AdventureWorks2022'
	and IC.TABLE_SCHEMA = @P1
	and t.TABLE_TYPE = 'BASE TABLE'
order by
	t.TABLE_NAME",
                    &[schema_name],
                )
                .await
                .map_err(|e| StopTrek::Tiberius(e))?
                .into_first_result()
                .await
                .map_err(|e| StopTrek::Tiberius(e))?;
            for row in query {
                let column_name: &str = row.get("column_name").unwrap();
                let data_type: &str = row.get("data_type").unwrap();
                let length_field: i32 = row.get("length_field").unwrap();
            }
        }
        _ => {
            println!(
                "You are aiming to use a SQL Server Connection, please retry and verify your connection list!"
            )
        }
    }
    Ok(())
}
