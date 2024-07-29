use bitwarden_db::{params, Database, DatabaseTrait};
use wasm_bindgen::prelude::*;

/*
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
    */

#[wasm_bindgen(js_name = runTests)]
pub async fn run_tests() {
    let db = Database::default().await.unwrap();

    db.execute("CREATE TABLE test (id INTEGER PRIMARY KEY, name TEXT)", [])
        .await
        .unwrap();

    db.execute("INSERT INTO test (name) VALUES (?)", params!["abc"])
        .await
        .unwrap();

    #[derive(Debug, PartialEq)]
    struct Test {
        id: i64,
        name: String,
    }

    let rows = db
        .query_map("SELECT * FROM test", |row| {
            Ok(Test {
                id: row.get(0)?,
                name: row.get(1)?,
            })
        })
        .await
        .expect("To not error on select");

    assert_eq!(
        rows,
        vec![Test {
            id: 1,
            name: "abc".to_string()
        }]
    );

    print!("Ran tests");
}
