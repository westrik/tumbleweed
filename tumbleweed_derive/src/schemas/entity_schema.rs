use crate::diagnostic::Diagnostic;
use serde_derive::Deserialize;
use toml::value::Table;

#[derive(Deserialize, Debug)]
struct TomlEntitySchema {
    pub authenticating_entities: Option<Vec<String>>,
    pub entities: Table,
}

#[derive(Debug, PartialEq)]
pub struct EntitySchema {
    pub authenticating_entities: Vec<String>,
    pub entities: Vec<Entity>,
}

#[derive(Debug, PartialEq)]
pub struct Entity {
    pub name: String,
    pub api_id_prefix: Option<String>,
    pub fields: Vec<Field>,
}

#[derive(Debug, PartialEq)]
pub struct Field {
    pub field_name: String,
    pub field_type: FieldType,
    pub required: bool,
}

#[derive(Deserialize, Debug, PartialEq)]
pub enum FieldType {
    BigInt,
    Int,
    JsonBlob,
    PasswordHash,
    String,
    UtcTimestamp,
}

impl EntitySchema {
    #[allow(dead_code)]
    fn from(data: &str) -> Result<Self, Diagnostic> {
        let _schema: TomlEntitySchema = toml::from_str(data)
            .map_err(|err| Diagnostic::error(format!("Failed to parse TOML: {:#?}", err)))?;
        Ok(EntitySchema {
            authenticating_entities: vec![],
            entities: vec![Entity {
                name: "".to_string(),
                api_id_prefix: None,
                fields: vec![Field {
                    field_name: "fake".to_string(),
                    field_type: FieldType::BigInt,
                    required: false,
                }],
            }],
        })
    }
}

#[cfg(test)]
pub mod entity_schemas {
    use super::*;

    #[test]
    fn deserialize_entity_schema_from_toml() {
        match EntitySchema::from(
            r#"
            [entities.user]
            api_id_prefix = "usr"

            [entities.user.fields]
            name = { type = "string" }
            email = { type = "string", required = true }
            password = { type = "password_hash", required = true }
        "#,
        ) {
            Ok(_schema) => {}
            Err(err) => err.emit(),
        }

        // TODO: deserialize Table as an Entity

        // for entity_spec in schema.entities.iter() {
        //     let entity = match entity_spec.1 {
        //         Value::Table(et) => {
        //             let api_id_prefix = match et.get("api_id_prefix") {
        //                 Some(prefix) => match prefix {
        //                     Value::String(prefix_str) => Some(prefix_str.to_string()),
        //                     _ => panic!("Invalid value for api_id_prefix"),
        //                 },
        //                 None => None,
        //             };
        //             let fields = match et.get("fields") {
        //                 Some(fields) => match fields {
        //                     Value::Table(fields_table) => {
        //                         println!("{:#?}", fields_table);
        //
        //                         let mut fields_spec: Vec<Field> = vec![];
        //                         for (field_name, data) in fields_table.iter() {
        //                             fields_spec.push(match data {
        //                                 Value::Table(field_table) => {
        //                                     let field_type = match field_table.get("type").unwrap()
        //                                     {
        //                                         Value::String(type_str) => {
        //                                             let type_str = type_str.to_string();
        //                                             match type_str.as_str() {
        //                                                 "big_int" => FieldType::BigInt,
        //                                                 "int" => FieldType::Int,
        //                                                 "json_blob" => FieldType::JsonBlob,
        //                                                 "password_hash" => FieldType::PasswordHash,
        //                                                 "string" => FieldType::String,
        //                                                 "utc_timestamp" => FieldType::UtcTimestamp,
        //                                                 _ => panic!("Invalid field type"),
        //                                             }
        //                                         }
        //                                         _ => panic!("Invalid value for field type"),
        //                                     };
        //                                     let required = match field_table.get("required") {
        //                                         Some(val) => match val {
        //                                             Value::Boolean(required_bool) => *required_bool,
        //                                             _ => panic!(
        //                                                 "Expected boolean for 'required' field"
        //                                             ),
        //                                         },
        //                                         _ => false,
        //                                     };
        //                                     Field {
        //                                         field_name: field_name.to_string(),
        //                                         field_type,
        //                                         required,
        //                                     }
        //                                 }
        //                                 _ => panic!("Invalid value for field"),
        //                             });
        //                         }
        //                         Some(fields_spec)
        //                     }
        //                     _ => panic!("Invalid value for fields"),
        //                 },
        //                 None => None,
        //             };
        //             Entity {
        //                 name: entity_spec.0.to_string(),
        //                 api_id_prefix,
        //                 fields: fields.unwrap(),
        //             }
        //         }
        //         _ => panic!("Unexpected toml value"),
        //     };
        //     println!("got entity => {:#?}", entity);
        // }

        // TODO: deserialize Table as a Field

        // assert_eq!(schema.entities, vec!(Entity {
        //     fields: vec!(Field {
        //         identifier: "name".to_string(),
        //         field_type: FieldType::String,
        //         required: None
        //     })
        // }));
    }

    #[test]
    fn test_a_thingy() {
        let schema: TomlEntitySchema = toml::from_str(
            r#"
            [entities.user]
            api_id_prefix = "usr"
            fields = { name = { type = "string" } }
        "#,
        )
        .unwrap();
        for entity_spec in schema.entities.iter() {
            println!("{} => {:#?}", entity_spec.0, entity_spec.1);
        }
    }
}
