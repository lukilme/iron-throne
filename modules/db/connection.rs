use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use lazy_static::lazy_static;
use std::env;
use std::sync::Mutex;
use thiserror::Error;


#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("Variável de ambiente DATABASE_URL não foi definida")]
    MissingDatabaseUrl,
    
    #[error("Falha ao conectar ao banco de dados: {0}")]
    ConnectionError(#[from] diesel::result::ConnectionError),
}

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

    pub fn get_connection(&mut self) -> Result<&PgConnection, DatabaseError> {
        if self.connection.is_none() {
            self.connect()?;
        }
        Ok(self.connection.as_ref().expect("Conexão deve estar estabelecida neste ponto"))
    }
}

lazy_static! {
    pub static ref SINGLETON_INSTANCE: Mutex<Database> = Mutex::new(Database::new());
}

pub fn get_shared_connection() -> Result<std::sync::MutexGuard<'static, Database>, DatabaseError> {
    let mut db_instance = SINGLETON_INSTANCE.lock().expect("Falha ao obter o lock do singleton");
    db_instance.get_connection()?;
    Ok(db_instance)
}
