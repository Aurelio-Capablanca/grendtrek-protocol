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
    static ref PRESCISION_TYPES: Vec<String> = vec!("NUMERIC".to_string(), "DECIMAL".to_string());
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

fn build_column(fields: &DataSchema) -> String {
    let mut ddl_for_tables = String::new();
    let type_field: &str;
    if fields
        .get_constraint_type()
        .eq_ignore_ascii_case("PRIMARY KEY")
    {
        type_field = "SERIAL";
    } else {
        type_field = TYPE_MAPPING
            .get(fields.get_data_type())
            .map(|s| s.as_str())
            .unwrap_or("");
    }
    ddl_for_tables.push_str("\"");
    ddl_for_tables.push_str(&fields.get_column_name().replace(" ", "_"));
    ddl_for_tables.push_str("\"");
    ddl_for_tables.push_str(" ");
    ddl_for_tables.push_str(type_field);

    if fields.get_length_field() > 0 {
        ddl_for_tables.push_str("(");
        ddl_for_tables.push_str(&fields.get_length_field().to_string());
        ddl_for_tables.push_str(")")
    }
    if fields.get_numeric_precision() > 0
        && fields.get_numeric_scale() > 0
        && PRESCISION_TYPES.contains(&type_field.to_string())
    {
        ddl_for_tables.push_str("(");
        ddl_for_tables.push_str(&fields.get_numeric_precision().to_string());
        ddl_for_tables.push_str(",");
        ddl_for_tables.push_str(&fields.get_numeric_scale().to_string());
        ddl_for_tables.push_str(")")
    }
    if "NO"
        .to_string()
        .eq_ignore_ascii_case(fields.get_is_nullable())
    {
        ddl_for_tables.push_str(" NOT NULL")
    }
    ddl_for_tables
}

pub fn translate_ddl(structure_table: &Vec<DataSchema>) -> Result<Vec<String>, StopTrek> {
    let mut fields_table: HashMap<String, Vec<&DataSchema>> = HashMap::new();
    let mut ddl_sentences: Vec<String> = Vec::new();
    structure_table.into_iter().for_each(|data| {
        let table_name = data.get_table_name().to_string();
        fields_table
            .entry(table_name)
            .or_insert_with(Vec::new)
            .push(data);
    });
    fields_table.iter().for_each(|(table_name, fields)| {
        let mut ddl_tables = String::new();
        ddl_tables.push_str("create table ");
        ddl_tables.push_str("\"");
        ddl_tables.push_str(&table_name);
        ddl_tables.push_str("\"");
        ddl_tables.push_str("( ");
        let columns = &fields
            .iter()
            .map(|data| build_column(data))
            .collect::<Vec<String>>()
            .join(", ");
        let constraint = fields
            .iter()
            .filter(|x| x.get_constraint_type().len() > 0)
            .filter(|x| !x.get_constraint_type().eq_ignore_ascii_case("FOREIGN KEY"))
            .collect::<Vec<&&DataSchema>>();
        let constraints = vec![constraint];
        let construct_constraint = constraints
            .iter()
            .map(|data| {
                let mut constraint_name: String = String::new();
                let mut constraint_type: String = String::new();
                let fields_use = data
                    .iter()
                    .map(|x| {
                        constraint_name = x.get_constraint_name().to_string();
                        constraint_type = x.get_constraint_type().to_string();
                        format!("\" {:?} \"", x.get_column_name())
                    })
                    .collect::<Vec<String>>()
                    .join(",");
                format!(
                    "CONSTRAINT \"{:?}\" {:?} {:?}",
                    constraint_name, constraint_type, fields_use
                )
            })
            .collect::<Vec<String>>()
            .join("");
        ddl_tables.push_str(columns);
        if construct_constraint.is_empty() {
            ddl_tables.push_str("");
        } else {
            ddl_tables.push_str(", ");
            ddl_tables.push_str(&construct_constraint);
        }
        ddl_tables.push_str(" );");
        ddl_sentences.push(ddl_tables);
    });
    Ok(ddl_sentences)
}
