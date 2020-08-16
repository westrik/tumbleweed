use tumbleweed_derive::generate_structs_for_schema;

#[test]
fn basic_entity_definition() {
    generate_structs_for_schema!("tumbleweed_derive/tests/data/example_app.toml");
}
