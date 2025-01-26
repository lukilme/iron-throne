pub mod lib_module {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/modules/db/connection.rs"
    ));
}

use dotenvy::dotenv_override;
use lib_module::Database;
#[allow(dead_code)]
use lib_module::DatabaseError;
use std::env;

#[test]
fn test_successful_connection() {
    dotenv_override().ok();

    let _database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL deve estar definida para este teste");

    let mut db = Database::new();
    let result = db.connect();

    assert!(
        result.is_ok(),
        "Esperava-se sucesso na conexão, mas obteve-se: {:?}",
        result
    );
    assert!(
        db.connection.is_some(),
        "A conexão deve estar presente após um sucesso"
    );
}
#[test]
fn test_execute_query() {
    let mut db = Database::new();

    let insert_query = "INSERT INTO \"Users\" (name) VALUES ('Alice'), ('Bob'), ('Charlie');";
    let result = db.execute_query(insert_query);

    assert!(result.is_ok());

    let rows_affected = result.unwrap();
    assert!(
        rows_affected > 0,
        "Esperava-se que mais de 0 linhas fossem afetadas, mas obtivemos: {}",
        rows_affected
    );

    let select_query = "SELECT * FROM \"Users\";";
    let result = db.execute_query(select_query);
    println!("{:?}", result);
    assert!(result.is_ok());

    let rows_returned = result.unwrap();
    assert!(
        rows_returned > 0,
        "Esperava-se que mais de 0 linhas fossem retornadas, mas obtivemos: {}",
        rows_returned
    );
}

#[test]
fn test_singleton_connection() {
    dotenv_override().ok();

    let result = lib_module::get_shared_connection();
    assert!(
        result.is_ok(),
        "Esperava-se uma conexão compartilhada bem-sucedida, mas obteve-se: {:?}",
        result.err()
    );
}
#[allow(dead_code)]
#[test]
fn test_reuse_existing_connection() {
    dotenv_override().ok();
    let mut db = Database::new();
    let _ = db.connect().expect("Conexão inicial deve ser bem-sucedida");

    let conn = db.get_connection();
    assert!(
        conn.is_ok(),
        "Esperava-se reutilizar a conexão existente, mas obteve-se: {:?}",
        conn.err()
    );
}
#[allow(dead_code)]
//#[test]
fn test_invalid_database_url() {
    let original_value = env::var("DATABASE_URL").ok();
    println!("Valor original de DATABASE_URL: {:?}", original_value);

    env::set_var(
        "DATABASE_URL",
        "postgres://usuario_invalido:senha_invalida@localhost/db_invalido",
    );

    let mut db = Database::new();
    let result = db.connect();
    assert!(
        matches!(result, Err(DatabaseError::ConnectionError(_))),
        "Esperava-se um erro ConnectionError, mas obteve-se: {:?}",
        result
    );
    //INFERNNOOOOOOOOOO DE LINGUAGEM
    if let Some(value) = original_value {
        env::set_var("DATABASE_URL", value);
    } else {
        env::remove_var("DATABASE_URL");
        println!("Removendo DATABASE_URL, pois não estava definida originalmente.");
    }
    let result = db.connect();
    let restored_value = env::var("DATABASE_URL").ok();
    println!(
        "Valor restaurado de DATABASE_URL: {:?} and result {:?}",
        restored_value, result
    );
}

#[allow(dead_code)]
fn main() {
    dotenv_override().ok();
}
