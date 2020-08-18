use crate::diagnostic::Diagnostic;
use serde_derive::Deserialize;
use toml::value::Table;
use toml::Value;

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
    Password,
    String,
    UtcTimestamp,
}

impl Field {
    fn from_data(field_name: &str, field_val: &Value) -> Result<Self, Diagnostic> {
        let field_table = match field_val {
            Value::Table(ft) => Ok(ft),
            _ => Err(Diagnostic::error(format!(
                "Expected table for field {}",
                field_name
            ))),
        }?;
        let field_type = {
            let type_val = match field_table.get("type") {
                Some(tv) => Ok(tv),
                None => Err(Diagnostic::error(format!(
                    "Expected type for field {}",
                    field_name
                ))),
            }?;
            let type_str = match type_val {
                Value::String(ts) => Ok(ts),
                _ => Err(Diagnostic::error(format!(
                    "Expected type to be string for field {}",
                    field_name
                ))),
            }?;
            match type_str.as_str() {
                "big_int" => Ok(FieldType::BigInt),
                "int" => Ok(FieldType::Int),
                "json_blob" => Ok(FieldType::JsonBlob),
                "password" => Ok(FieldType::Password),
                "string" => Ok(FieldType::String),
                "utc_timestamp" => Ok(FieldType::UtcTimestamp),
                _ => Err(Diagnostic::error(format!(
                    "Invalid field type: {}",
                    type_str
                ))),
            }?
        };
        let required = match field_table.get("required") {
            Some(val) => match val {
                Value::Boolean(required_bool) => Ok(*required_bool),
                _ => Err(Diagnostic::error(format!(
                    "Expected required to be boolean for field {}",
                    field_name
                ))),
            },
            _ => Ok(false),
        }?;
        Ok(Field {
            field_name: field_name.to_string(),
            field_type,
            required,
        })
    }
}

impl Entity {
    fn from_data(entity_name: &str, entity_val: &Value) -> Result<Self, Diagnostic> {
        let entity_table = match entity_val {
            Value::Table(et) => Ok(et),
            _ => Err(Diagnostic::error(format!(
                "Expected table for entity {}",
                entity_name
            ))),
        }?;
        let api_id_prefix = match entity_table.get("api_id_prefix") {
            Some(prefix) => match prefix {
                Value::String(prefix_str) => Ok(Some(prefix_str.to_string())),
                _ => Err(Diagnostic::error("Invalid value for api_id_prefix")),
            },
            None => Ok(None),
        }?;
        let fields = match entity_table.get("fields") {
            Some(fields) => match fields {
                Value::Table(fields_table) => {
                    let mut fields: Vec<Field> = vec![];
                    for (field_name, data) in fields_table.iter() {
                        fields.push(Field::from_data(field_name, data)?)
                    }
                    Some(fields)
                }
                _ => panic!("Invalid value for fields"),
            },
            None => None,
        };
        Ok(Entity {
            name: entity_name.to_string(),
            api_id_prefix,
            fields: fields.unwrap(),
        })
    }
}

impl EntitySchema {
    pub fn from_str(data: &str) -> Result<Self, Diagnostic> {
        let schema: TomlEntitySchema = toml::from_str(data)
            .map_err(|err| Diagnostic::error(format!("Failed to parse TOML: {:#?}", err)))?;
        let mut entities: Vec<Entity> = vec![];
        for (entity_name, data) in schema.entities.iter() {
            entities.push(Entity::from_data(entity_name, data)?);
        }
        Ok(EntitySchema {
            authenticating_entities: vec![],
            entities,
        })
    }
}

#[cfg(test)]
pub mod entity_schemas {
    use super::*;

    #[test]
    fn deserialize_entity_schema_from_toml() {
        match EntitySchema::from_str(
            r#"
            [entities.user]
            api_id_prefix = "usr"

            [entities.user.fields]
            name = { type = "string" }
            email = { type = "string", required = true }
            password = { type = "password_hash", required = true }
        "#,
        ) {
            Ok(schema) => println!("{:#?}", schema),
            Err(err) => err.emit(),
        }
    }
}
