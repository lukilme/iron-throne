use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use lazy_static::lazy_static;
use std::env;
use std::sync::{Mutex, MutexGuard};
use thiserror::Error;

#[allow(dead_code)]
#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("Variável de ambiente DATABASE_URL não foi definida")]
    MissingDatabaseUrl,

    #[error("Falha ao conectar ao banco de dados: {0}")]
    ConnectionError(#[from] diesel::result::ConnectionError),

    #[error("Erro ao executar a query: {0}")]
    QueryExecutionError(#[from] diesel::result::Error),
}

#[allow(dead_code)]
pub struct Database {
    pub connection: Option<PgConnection>,
}

impl Database {
    pub fn new() -> Self {
        Database { connection: None }
    }

    
    pub fn connect(&mut self) -> Result<(), DatabaseError> {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").map_err(|_| DatabaseError::MissingDatabaseUrl)?;
        let connection = PgConnection::establish(&database_url)?;
        self.connection = Some(connection);
        Ok(())
    }


    #[allow(dead_code)]
    pub fn get_connection(&mut self) -> Result<&mut PgConnection, DatabaseError> {
        if self.connection.is_none() {
            self.connect()?;
        }
        Ok(self
            .connection
            .as_mut()
            .expect("Conexão deve estar estabelecida neste ponto"))
    }

    #[allow(dead_code)]
    pub fn execute_query(&mut self, query: &str) -> Result<usize, DatabaseError> {
        use diesel::RunQueryDsl;

        let connection = self.get_connection()?;
        diesel::sql_query(query)
            .execute(connection)
            .map_err(DatabaseError::QueryExecutionError)
    }
}


lazy_static! {
    pub static ref SINGLETON_INSTANCE: Mutex<Database> = Mutex::new(Database::new());
}
#[allow(dead_code)]
pub fn get_shared_connection() -> Result<MutexGuard<'static, Database>, DatabaseError> {
    let mut db_instance = SINGLETON_INSTANCE.lock().expect("Falha ao obter o lock do singleton");
    db_instance.get_connection()?;
    Ok(db_instance)
}
