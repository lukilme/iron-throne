pub mod lib_module {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/modules/db/connection.rs"));
}

pub use lib_module::*;
use dotenvy::dotenv;
use std::env;
use std::sync::Once;

static INIT: Once = Once::new();

fn initialize_env() {
    INIT.call_once(|| {
        dotenv().ok();
    });
}

#[test]
fn test_missing_database_url() {
    initialize_env();
    
    let original_value = env::var("DATABASE_URL").ok();
    env::remove_var("DATABASE_URL");
    
    let result = Database::new().connect();
    assert!(matches!(result, Err(DatabaseError::MissingDatabaseUrl)));
    
    if let Some(value) = original_value {
        env::set_var("DATABASE_URL", value);
    }
}

#[test]
fn test_successful_connection() {
    initialize_env();
    
    let _database_url = env::var("DATABASE_URL").expect("DATABASE_URL deve estar definida para este teste");
    
    let mut db = Database::new();
    let result = db.connect();
    
    assert!(result.is_ok(), "A conexão com o banco de dados deve ser bem-sucedida");
    assert!(db.connection.is_some(), "A conexão deve estar presente");
}

#[test]
fn test_singleton_connection() {
    initialize_env();
    
    let result = get_shared_connection();
    assert!(result.is_ok(), "A conexão compartilhada deve ser bem-sucedida");
}

#[test]
fn test_reuse_existing_connection() {
    initialize_env();
    
    let mut db = Database::new();
    let _ = db.connect().expect("Conexão inicial deve ser bem-sucedida");
    
    let conn = db.get_connection();
    assert!(conn.is_ok(), "Deve ser possível reutilizar a conexão existente");
}

#[test]
fn test_invalid_database_url() {
    initialize_env();
    
    let original_value = env::var("DATABASE_URL").ok();
    env::set_var("DATABASE_URL", "postgres://usuario_invalido:senha_invalida@localhost/db_invalido");
    
    let mut db = Database::new();
    let result = db.connect();
    assert!(matches!(result, Err(DatabaseError::ConnectionError(_))));
    
    if let Some(value) = original_value {
        env::set_var("DATABASE_URL", value);
    } else {
        env::remove_var("DATABASE_URL");
    }
}