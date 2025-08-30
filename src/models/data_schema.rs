pub struct DataSchema{
    column_name: Option<String>,
    data_type: Option<String>,
    length_field: Option<i32>,
    description: Option<String>,
    constraint_name: Option<String>,
    constraint_type: Option<String>,
    is_nullable: Option<String>,
    table_name: Option<String>,
    table_schema: Option<String>,
    numeric_precision: Option<i32>,
    numeric_scale: Option<i32>,
    schema_name: Option<String>
}

//trait DataSchema