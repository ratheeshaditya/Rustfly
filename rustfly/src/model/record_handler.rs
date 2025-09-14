use std::collections::HashMap;
use crate::model::ModelSchema;

#[derive(Debug)]
pub struct RecordHandler {
    pub records: Vec<HashMap<String, String>>,
    pub schema: ModelSchema,
}

impl RecordHandler {
    pub fn validate_schema(&self, field_values: &HashMap<String, String>) -> Result<(), Vec<String>> {
        // Function checks for missing fields
        let mut missing_fields = Vec::new();

        for column in &self.schema.columns {
            if column.is_required && !field_values.contains_key(&column.name) {
                missing_fields.push(column.name.clone());
            }
        }

        if missing_fields.is_empty() {
            Ok(())
        } else {
            Err(missing_fields)
        }
    }

    pub fn find(&self, column: String, value: String) -> Option<&HashMap<String, String>>   {
        // Function finds a record with a value         
        let mut result = None;

        for record in &self.records{
            if record.get(&column) == Some(&value){
                result = Some(record)
            }
        }
        result
    }

    pub fn find_all(&self, column: String, value: String) { 
        let mut result = Vec::new();

        for record in &self.records{
            if record.get(&column) == Some(&value){
                result.push(record);
            }
        }
    }

    pub fn find_where(&self, params: &HashMap<String, String>) {         
        let mut result = Vec::new();
        let param_keys: Vec<&String> = params.keys().collect();

        for record in &self.records {
                if params.iter().all(|(k, v)| record.get(k) == Some(v)) {
                    result.push(record);
                }
        }
        println!("{:#?}", result);
    }

    pub fn validate_record(&self, field_values: &HashMap<String, String>) -> Result<(), Vec<String>> {
        RecordHandler::validate_schema(&self, field_values)
    }
}