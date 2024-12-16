use serde::{Deserialize, Serialize};
use diesel::prelude::*;
pub mod lib_module {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/modules/db/dao.rs"));
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/modules/db/connection.rs"));
}
pub use lib_module::*;
use crate::schema::posts;

#[derive(Queryable, Identifiable, Insertable, AsChangeset, Serialize, Deserialize, Clone)]
#[table_name = "posts"]
pub struct Post {
    pub id: Option<i32>,
    pub title: String,
    pub body: String,
    pub user_id: i32,
}

#[test]
fn test_dao() {
    let database_url = "postgres://username:password@localhost/db_name";
    let result = Database::new().connect();
    let conn = PgConnection::establish(&database_url).expect("Failed to connect to database");

    let user_dao = GenericDaoImpl::<User, users::table>::new();

   
    let new_user = User {
        id: None,
        name: "Jane Doe".to_string(),
        email: "jane.doe@example.com".to_string(),
    };
    let created_user = user_dao.create(&conn, &new_user).expect("Failed to create user");
    println!("Created User: {:?}", created_user);

   
    let users = user_dao.list(&conn).expect("Failed to list users");
    println!("Users: {:?}", users);

    // Buscar um usuário pelo ID
    let user = user_dao.read(&conn, created_user.id.unwrap()).expect("User not found");
    println!("Fetched User: {:?}", user);

    
    let updated_user = User {
        id: user.id,
        name: "Jane Smith".to_string(),
        email: "jane.smith@example.com".to_string(),
    };
    let user = user_dao.update(&conn, &updated_user).expect("Failed to update user");
    println!("Updated User: {:?}", user);

    user_dao.delete(&conn, user.id.unwrap()).expect("Failed to delete user");
    println!("User deleted.");
}
