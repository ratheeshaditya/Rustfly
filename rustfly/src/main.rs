
 use std::collections::HashMap;
#[derive(Debug)]

struct Field{
    name: String,
    field_type: String,
    is_required: bool,
    is_unique: bool,
}

#[derive(Debug)]
struct ModelSchema{
    fields: Vec<Field>,
}
#[derive(Debug)]
struct Model{
    class_name: String,
    schema: ModelSchema,
}

impl Model {
    fn define_schema(params: HashMap <String,Field>) -> ModelSchema {
       let mut fields: Vec<Field> = Vec::new();

       for (_, field) in params.into_iter() {
        fields.push(field)
       }

       ModelSchema { fields }
    }

    fn new(name: String, params: HashMap <String,Field>) -> Model {
        let model_schema: ModelSchema = Model::define_schema(params);

        Model {
            class_name: name.to_string(),
            schema: model_schema
        }
    }



}
fn main() {
   
    let mut user_fields = HashMap::new();
    user_fields.insert("user".to_string(), Field { name:"user".to_string(), field_type: "int".to_string(), is_required: true, is_unique: true });
    user_fields.insert("email".to_string(), Field { name: "email".to_string(), field_type: "string".to_string(), is_required: true, is_unique: true });

    let user_model = Model::new("User".to_string(), user_fields);
    println!("{:#?}", user_model);
}
