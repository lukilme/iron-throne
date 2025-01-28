use serde::{Deserialize, Serialize};

pub mod lib_module {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/modules/db/dao.rs"));
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/modules/db/connection.rs"
    ));
}

use dotenvy::dotenv_override;
use lib_module::Database;
use lib_module::DAO;

use std::env;

#[derive(Serialize, Deserialize, Default)]
struct Userios1 {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
    morte: u64
}

impl DAO for Userios1 {}

fn setup() -> Database {
    dotenv_override().ok();
    let _database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL deve estar definida para este teste");
    let mut db = Database::new();
    let _ = db.connect();
    return db;
}

#[test]
fn create_table() {}

#[test]
fn insert_table() {
    let mut db = setup();
    println!("{}", db);
    let _ = db.connect();

    let user1 = Userios1 {
        active: true,
        username: String::from("someusername12332"),
        email: String::from("someone@exampl2e.com"),
        sign_in_count: 1,
        morte:3
    };
    user1.create();
    user1.persist();
}

#[test]
fn create_a_simple_object() {}

#[allow(dead_code)]
fn main() {
    println!("ok");
}
