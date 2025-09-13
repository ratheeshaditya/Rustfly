use std::collections::HashMap;
use crate::model::ModelSchema;

#[derive(Debug)]
pub struct RecordHandler {
    pub records: Vec<HashMap<String, String>>,
}

impl RecordHandler {
    pub fn validate_schema(&self, schema: &ModelSchema, field_values: &HashMap<String, String>) -> Result<(), Vec<String>> {
        // Function checks for missing fields
        let mut missing_fields = Vec::new();

        for column in &schema.columns {
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

    pub fn find(&self, column: String, value: String, schema: &ModelSchema) -> Option<&HashMap<String, String>>   {
        // Function finds a record with a value
        let column_names: Vec<&String> = schema
            .columns
            .iter()
            .map(|column| &column.name) // clone if column.name is String
            .collect();
  
      
        let mut result = None;

        for record in &self.records{
            if record.get(&column) == Some(&value){
                result = Some(record)
            }
        }
        result
    }

    pub fn find_all(&self, column: String, value: String, schema: &ModelSchema) { 
        let column_names: Vec<&String> = schema
            .columns
            .iter()
            .map(|column| &column.name) // clone if column.name is String
            .collect();
        
        let mut result = Vec::new();

        for record in &self.records{
            if record.get(&column) == Some(&value){
                result.push(record);
            }
        }
        // result;
        println!("{:?}", result);
    }
    pub fn validate_record(&self, schema: &ModelSchema, field_values: &HashMap<String, String>) -> Result<(), Vec<String>> {
        RecordHandler::validate_schema(&self, schema, field_values)
    }
}