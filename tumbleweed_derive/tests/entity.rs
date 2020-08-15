use tumbleweed_derive::Entity;

#[test]
fn basic_entity_definition() {
    #[derive(Entity)]
    struct MyStruct {
        foo: i32,
        bar: i32,
    }

    // let conn = connection();
    // let data = select(sql::<(Integer, Integer)>("1, 2")).get_result(&conn);
    // assert_eq!(Ok(MyStruct { foo: 1, bar: 2 }), data);
}
