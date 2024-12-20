
pub mod lib_module {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/modules/db/dao.rs"));
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/modules/db/connection.rs"));
}



#[test]
fn test_dao() {
    println!("No");
}
