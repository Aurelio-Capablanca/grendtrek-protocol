#[derive(Debug,Default)]
pub struct DataSchema {
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
    numeric_scale: Option<i32>
}

impl DataSchema {
    pub fn new(
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
        numeric_scale: Option<i32>
    ) -> Self {
        Self {
            column_name,
            data_type,
            length_field,
            description,
            constraint_name,
            constraint_type,
            is_nullable,
            table_name,
            table_schema,
            numeric_precision,
            numeric_scale,
        }
    }
}
