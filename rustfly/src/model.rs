use std::collections::HashMap;

pub mod record_handler;
use record_handler::RecordHandler;

#[derive(Debug, Clone)]
pub struct ColumnField {
    pub name: String,
    pub field_type: String,
    pub is_required: bool,
    pub is_unique: bool,
}

#[derive(Debug, Clone)]
pub struct ModelSchema {
    columns: Vec<ColumnField>,
}

#[derive(Debug)]
pub struct Model {
    class_name: String,
    pub schema: ModelSchema,
    pub record_handler: RecordHandler,
}

impl Model {
    pub fn define_schema(params: HashMap<String, ColumnField>) -> ModelSchema {
        let fields: Vec<ColumnField> = params.into_values().collect();
        ModelSchema { columns: fields }
    }

    pub fn new(name: String, params: HashMap<String, ColumnField>) -> Self {
        let model_schema = Model::define_schema(params);

        Model {
            class_name: name,
            schema: model_schema,
            record_handler: RecordHandler { records: Vec::new() },
        }
    }

    pub fn show_records(&self) {
        println!("{:#?}", self.record_handler.records);
    }

    pub fn create_record(&mut self, data: HashMap<String, String>) {
        match self.record_handler.validate_record(&self.schema, &data)  {
            Ok(_) => self.record_handler.records.push(data),
            Err(missing) => eprintln!("Error: missing required fields: {:?}", missing),
        }
    }
}