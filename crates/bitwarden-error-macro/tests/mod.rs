use bitwarden_error::prelude::*;

#[test]
fn flattens_basic_enums() {
    #[derive(FlatError)]
    enum Errors {
        Foo,
        Bar,
        Baz,
    }

    let foo = Errors::Foo;
    let bar = Errors::Bar;
    let baz = Errors::Baz;

    assert_eq!(foo.get_variant(), "Foo");
    assert_eq!(bar.get_variant(), "Bar");
    assert_eq!(baz.get_variant(), "Baz");

    assert_eq!(foo.get_message(), "Foo");
    assert_eq!(bar.get_message(), "Bar");
    assert_eq!(baz.get_message(), "Baz");
}

#[test]
fn flattens_enums_with_fields() {
    #[derive(FlatError)]
    enum Errors {
        #[allow(dead_code)]
        Foo(String),
        #[allow(dead_code)]
        Bar(u32),
        Baz,
    }

    let foo = Errors::Foo("hello".to_string());
    let bar = Errors::Bar(42);
    let baz = Errors::Baz;

    assert_eq!(foo.get_variant(), "Foo");
    assert_eq!(bar.get_variant(), "Bar");
    assert_eq!(baz.get_variant(), "Baz");

    // The message is always "Error: <variant>"
    // TODO: Add support for getting the message from the fields
    // or maybe just remove get_message and rely on ToString
    assert_eq!(foo.get_message(), "Error: Foo");
    assert_eq!(bar.get_message(), "Error: Bar");
    assert_eq!(baz.get_message(), "Baz");
}
