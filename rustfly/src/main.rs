use std::collections::HashMap;
// use crate::model::ColumnField;
mod model;

use model::{ColumnField, Model};

fn main() {
    let mut user_fields = HashMap::new();
    user_fields.insert("user".to_string(), ColumnField {
        name: "user".to_string(),
        field_type: "int".to_string(),
        is_required: true,
        is_unique: true,
    });
    user_fields.insert("email".to_string(), ColumnField {
        name: "email".to_string(),
        field_type: "string".to_string(),
        is_required: true,
        is_unique: true,
    });

    let mut user_model = Model::new("User".to_string(), user_fields);
    // println!("{:#?}", user_model);

    let mut record = HashMap::new();
    record.insert("user".to_string(), "123".to_string());
    record.insert("email".to_string(), "test@example.com".to_string());
    user_model.create_record(record);

    // user_model.show_records();
        // println!("{:#?}", user_model);
    // user_model.record_handler.find("email".to_string(), "test@example.com".to_string(), &user_model.schema);
    // user_model.record_handler.find_all("email".to_string(), "test@example.com".to_string());

    let mut new_record = HashMap::new();
    new_record.insert("user".to_string(), "123456".to_string());
    new_record.insert("email".to_string(), "test@example.com".to_string());
    user_model.create_record(new_record);
    
    user_model.record_handler.find_where(
        &HashMap::from([("email".to_string(),"test@example.com".to_string())])
    )
    
}
