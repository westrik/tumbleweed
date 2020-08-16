use tumbleweed_derive::generate_structs_for_schema;

#[test]
fn test_generate_structs_for_schema() {
    generate_structs_for_schema!("tumbleweed_derive/tests/data/example_app.toml");

    let user = entities::user {
        email: "test@example.com".to_string(),
        name: "Test Person".to_string(),
        password: "password123".to_string(),
    };
    println!("{:#?}", user);
}
