use serde_derive::Deserialize;
use toml::map::Map;
use toml::value::Table;

#[derive(Deserialize, Debug)]
pub struct EntitySchema {
    authenticating_entities: Option<Vec<String>>,
    entities: Table,
}

#[derive(Debug, PartialEq)]
pub struct Entity {
    name: String,
    api_id_prefix: Option<String>,
    fields: Vec<Field>,
}

#[derive(Debug, PartialEq)]
pub struct Field {
    name: String,
    field_type: FieldType,
    required: Option<bool>,
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

#[cfg(test)]
pub mod entity_schemas {
    use super::*;
    use toml::Value;

    #[test]
    fn deserialize_entity_schema_from_toml() {
        let schema: EntitySchema = toml::from_str(
            r#"
            [entities.user]
            api_id_prefix = "usr"

            [entities.user.fields]
            name = { type = "string" }
            password = { type = "password_hash" }
        "#,
        )
        .unwrap();

        // TODO: deserialize Table as an Entity

        for entity_spec in schema.entities.iter() {
            let entity = match entity_spec.1 {
                Value::Table(et) => {
                    let api_id_prefix = match et.get("api_id_prefix") {
                        Some(prefix) => match prefix {
                            Value::String(prefix_str) => Some(prefix_str.to_string()),
                            _ => panic!("Invalid value for api_id_prefix"),
                        },
                        None => None,
                    };
                    let fields = match et.get("fields") {
                        Some(fields) => match fields {
                            Value::Table(fields_table) => {
                                println!("{:#?}", fields_table);

                                let mut fields_spec: Vec<Field> = vec![];
                                for (field_name, data) in fields_table.iter() {
                                    fields_spec.push(match data {
                                        Value::Table(field_table) => {
                                            let field_type = match field_table.get("type").unwrap()
                                            {
                                                Value::String(type_str) => {
                                                    let type_str = type_str.to_string();
                                                    match type_str.as_str() {
                                                        "big_int" => FieldType::BigInt,
                                                        "int" => FieldType::Int,
                                                        "json_blob" => FieldType::JsonBlob,
                                                        "password_hash" => FieldType::PasswordHash,
                                                        "string" => FieldType::String,
                                                        "utc_timestamp" => FieldType::UtcTimestamp,
                                                        _ => panic!("Invalid field type"),
                                                    }
                                                }
                                                _ => panic!("Invalid value for field type"),
                                            };
                                            Field {
                                                name: field_name.to_string(),
                                                field_type,
                                                required: None,
                                            }
                                        }
                                        _ => panic!("Invalid value for field"),
                                    });
                                }
                                Some(fields_spec)
                            }
                            _ => panic!("Invalid value for fields"),
                        },
                        None => None,
                    };
                    Entity {
                        name: entity_spec.0.to_string(),
                        api_id_prefix,
                        fields: fields.unwrap(),
                    }
                }
                _ => panic!("Unexpected toml value"),
            };
            println!("got entity => {:#?}", entity);
        }

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
        let schema: EntitySchema = toml::from_str(
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