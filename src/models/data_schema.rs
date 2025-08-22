pub struct DataSchema{
    column_name: String,
    data_type: String,
    length_field: i32,
    description: String,
    constraint_name: String,
    constraint_type: String,
    is_nullable: String,
    table_name: String,
    numeric_precision: i32,
    numeric_scale: i32,
    schema_name: String
}

//trait DataSchema