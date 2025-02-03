mod erro;
mod database;
mod prelude;
use std::env;

use dotenvy::dotenv_override;
use iron_throne_v2::prelude::*;  // Importação absoluta
use macros::ActiveRecord;
use iron_throne_v2::prelude::active_record::ActiveRecord;

#[derive(ActiveRecord)]
struct Usuario {
    #[primary_key]
    id: i64,
    name: String,
}


fn setup() -> database::Database {
    dotenv_override().ok();
    let _database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL deve estar definida para este teste");
    let mut db = database::Database::new();
    let _ = db.connect();
    return db;
}

fn main() {

    let db = setup();
    println!("{}", db);    
    let user = Usuario { id: 1, name: "Alice".to_string() };
    user.save().unwrap();
}