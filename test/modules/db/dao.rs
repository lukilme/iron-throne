use serde::{Serialize, Deserialize};

pub mod lib_module {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/modules/db/dao.rs"));
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/modules/db/connection.rs"));
}

use lib_module::Database;
use lib_module::DAO;
use dotenvy::dotenv_override;
use std::env;


fn setup()-> Database{
    dotenv_override().ok();
    let _database_url = env::var("DATABASE_URL").expect("DATABASE_URL deve estar definida para este teste");
    let mut db = Database::new();
    let _ = db.connect();

    return db;
}



#[test]
fn create_table(){
  
    let mut db = setup();
    let _ = db.connect();
    #[derive(Serialize, Deserialize, Default)]
    struct Treco {
        coisa: u32,
        septo: String
    }

    impl DAO for Treco {
        fn primary_key_field() -> String {
            "coisa".to_string()
        }
    }

    Treco::create();
}


#[test]
fn create_a_simple_object(){
    let mut db = setup();

    #[derive(Serialize, Deserialize, Default)]
    struct Usuario {

        id: u32,
        nome: String,
        idade: u32,
        ativo: bool,
    }

    let usuario = Usuario {
        id:1,
        nome: "Joao".to_string(),
        idade: 666,
        ativo: true,
    };
    
    impl DAO for Usuario {
        fn primary_key_field() -> String {
            "id".to_string()
        }
    }

    let query = Usuario::create_sql_query();
    println!("{}", query);

    let insert_query = query; 
    let result = db.execute_query(&insert_query);

    println!("{:?}",result);
    assert!(result.is_ok());
}

#[allow(dead_code)]
fn main(){
    println!("ok");
}