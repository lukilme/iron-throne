mod erro;
mod database;
mod prelude;

use iron_throne_v2::prelude::*;  // Importação absoluta
use macros::ActiveRecord;
use iron_throne_v2::prelude::active_record::ActiveRecord;

#[derive(ActiveRecord)]
struct User {
    #[primary_key]
    id: i64,
    name: String,
}

fn main() {
    let user = User { id: 1, name: "Alice".to_string() };
    user.save().unwrap();
}