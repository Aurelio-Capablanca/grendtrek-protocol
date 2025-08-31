use crate::common::grend_trek_error::StopTrek;
use crate::models::data_schema::DataSchema;
use lazy_static::lazy_static;
use std::collections::HashMap;

macro_rules ! hashmap {
( $ ( $ key: expr => $ val : expr), * ) => {{
let mut map =::std::collections::HashMap::new();
$ (map.insert( $ key, $ val); ) *
map
}};
}

lazy_static! {
    static ref TYPE_MAPPING: HashMap<String, String> = hashmap!(
        "nvarchar".to_string() => "VARCHAR".to_string(),
        "varchar".to_string() => "VARCHAR".to_string(),
        "int".to_string() => "INTEGER".to_string(),
        "tinyint".to_string() => "SMALLINT".to_string(),
        "datetime".to_string() => "TIMESTAMP".to_string(),
        "xml".to_string() => "TEXT".to_string(),
        "money".to_string() => "NUMERIC".to_string(),
        "uniqueidentifier".to_string() => "UUID".to_string(),
        "nchar".to_string() => "CHAR".to_string(),
        "geography".to_string() => "GEOGRAPHY".to_string(),
        "bit".to_string() => "BOOLEAN".to_string(),
        "smallmoney".to_string() => "NUMERIC".to_string(),
        "decimal".to_string() => "DECIMAL".to_string(),
        "hierarchyid".to_string() => "LTREE".to_string(),
        "smallint".to_string() => "SMALLINT".to_string(),
        "numeric".to_string() => "NUMERIC".to_string(),
        "date".to_string() => "DATE".to_string(),
        "time".to_string() => "TIME".to_string(),
        "varbinary".to_string() => "BIT".to_string()
    );
}

fn build_column(fields: DataSchema) -> String {
    let mut ddl_for_tables = String::new();

    "".to_string()
}

pub fn translate_ddl(structure_table: Vec<DataSchema>) -> Result<Vec<String>, StopTrek> {
    let mut fields_table: HashMap<String, Vec<DataSchema>> = HashMap::new();
    let mut ddl_sentences: Vec<String> = Vec::new();
    structure_table.into_iter().for_each(|data| {
        let table_name = data.get_table_name().to_string();
        fields_table
            .entry(table_name)
            .or_insert_with(Vec::new)
            .push(data);
    });
    fields_table.iter().for_each(|(table_name, fields )| {
        let mut ddl_tables = String::new().push("asdasdsad".to_string());
    });
    Ok(ddl_sentences)
}
