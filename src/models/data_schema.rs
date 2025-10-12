#[derive(Debug, Default)]
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
    numeric_scale: Option<i32>,
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
        numeric_scale: Option<i32>,
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

    pub fn empty_element() -> Self {
        Self {
            column_name: Some("".to_string()),
            data_type: Some("".to_string()),
            length_field: Some(0),
            description: Some("".to_string()),
            constraint_name: Some("".to_string()),
            constraint_type: Some("".to_string()),
            is_nullable: Some("".to_string()),
            table_name: Some("".to_string()),
            table_schema: Some("".to_string()),
            numeric_precision: Some(0),
            numeric_scale: Some(0),
        }
    }

    pub fn get_numeric_scale(&self) -> i32 {
        self.numeric_scale.unwrap_or(0)
    }

    pub fn get_numeric_precision(&self) -> i32 {
        self.numeric_precision.unwrap_or(0)
    }

    pub fn get_table_schema(&self) -> &str {
        self.table_schema.as_deref().unwrap_or("")
    }

    pub fn get_table_name(&self) -> &str {
        self.table_name.as_deref().unwrap_or("")
        //self.table_name.unwrap_or("".to_string())
    }

    pub fn get_is_nullable(&self) -> &str {
        self.is_nullable.as_deref().unwrap_or("")
    }

    pub fn get_constraint_type(&self) -> &str {
        self.constraint_type.as_deref().unwrap_or("")
    }

    pub fn get_constraint_name(&self) -> &str {
        self.constraint_name.as_deref().unwrap_or("")
    }

    pub fn _get_description(&self) -> &str {
        self.description.as_deref().unwrap_or("")
    }

    pub fn get_length_field(&self) -> i32 {
        self.length_field.unwrap_or(0)
    }

    pub fn get_column_name(&self) -> &str {
        self.column_name.as_deref().unwrap_or("")
    }

    pub fn get_data_type(&self) -> &str {
        self.data_type.as_deref().unwrap_or("")
    }
}
