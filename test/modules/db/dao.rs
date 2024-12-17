use serde::{Deserialize, Serialize};
use diesel::prelude::*;
pub mod lib_module {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/modules/db/dao.rs"));
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/modules/db/connection.rs"));
}
pub use lib_module::*;



#[test]
fn test_dao() {
    println!("No");
}
