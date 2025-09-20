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
    fn latest_id(&self) -> usize {
        self.record_handler.records.len() + 1
    }

    fn validate_id_field(params: &HashMap<String,ColumnField>) -> Result<(), String> {
        match params.get("id") {
            Some(f) if f.is_required && f.is_unique && f.field_type == "Integer" => Ok(()),
            Some(_) => Err("Invalid ID attributes. ID must be required, unique, and of type Integer".to_string()),
            None => Err("ID field missing".to_string()),
        }
    }

    pub fn define_schema(params: HashMap<String, ColumnField>) -> ModelSchema {
        let mut new_params = params;

        new_params.entry("id".to_string()).or_insert(
                                                    ColumnField {
                                                                    name: "id".to_string(),
                                                                    field_type: "Integer".to_string(),
                                                                    is_required: true,
                                                                    is_unique: true,
                                                                }
                                                );
                                                
        match Model::validate_id_field(&new_params) {
            Ok(()) => {
                let fields: Vec<ColumnField> = new_params.into_values().collect();
                ModelSchema { columns: fields }
            },
            Err(invalid_id_error_message) => { eprintln!("{}", invalid_id_error_message); std::process::exit(1); }
        }
    }

    pub fn new(name: String, params: HashMap<String, ColumnField>) -> Self {
        let model_schema: ModelSchema = Model::define_schema(params);

        Model {
            class_name: name,
            schema: model_schema.clone(),
            record_handler: RecordHandler { records: Vec::new(), schema: model_schema.clone()},
        }
    }

    pub fn show_records(&self) {
        println!("{:#?}", self.record_handler.records);
    }

    pub fn create_record(&mut self, mut data: HashMap<String, String>) {
        match self.record_handler.validate_record(&data)  {
            Ok(_) => match self.record_handler.validate_fields(&data) {
                Ok(_) => {
                    data.insert("id".to_string(), Model::latest_id(&self).to_string()); // Temporary ID assignment
                    self.record_handler.records.push(data);
                    println!("Record added successfully.");
                }
                Err(non_unique) => eprintln!("Error: non-unique fields: {:?}", non_unique),
            },
            Err(missing) => eprintln!("Error: missing required fields: {:?}", missing),
        }
    }
}