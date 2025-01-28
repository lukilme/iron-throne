use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::sql_types::Text;
use diesel::sql_query;
use diesel::RunQueryDsl;
use dotenvy::dotenv;
use lazy_static::lazy_static;
use std::env;
use std::fmt::{self, Display, Formatter};
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

#[allow(dead_code)]
impl Database {
    pub fn new() -> Self {
        Database { connection: None }
    }

    pub fn connect(&mut self) -> Result<(), DatabaseError> {
        dotenv().ok();
        let database_url =
            env::var("DATABASE_URL").map_err(|_| DatabaseError::MissingDatabaseUrl)?;

        let connection =
            PgConnection::establish(&database_url).map_err(DatabaseError::ConnectionError)?;

        self.connection = Some(connection);
        Ok(())
    }

    pub fn get_connection(&mut self) -> Result<&mut PgConnection, DatabaseError> {
        if self.connection.is_none() {
            self.connect()?;
        }
        self.connection.as_mut().ok_or_else(|| {
            DatabaseError::ConnectionError(diesel::ConnectionError::BadConnection(
                "Conexão não foi estabelecida".to_string(),
            ))
        })
    }

    pub fn execute_query(&mut self, query: &str) -> Result<usize, DatabaseError> {

        let connection = self.get_connection()?;

        let rows_affected = sql_query(query)
            .execute(connection)
            .map_err(DatabaseError::QueryExecutionError)?;

        println!(
            "Query executada com sucesso! Linhas afetadas: {}",
            rows_affected
        );

        Ok(rows_affected)
    }

    pub fn execute_query_with_params(
        &mut self,
        query: &str,
        params: Vec<String>,
    ) -> Result<usize, DatabaseError> {
        let connection = self.get_connection()?;
    
        let mut query_with_params = query.to_string();
        for (i, param) in params.iter().enumerate() {
            let placeholder = format!("${}", i + 1);
            query_with_params = query_with_params.replace(&placeholder, &format!("'{}'", param));
        }
    
        println!("Query com parâmetros substituídos: {}", query_with_params);
        println!("Parâmetros: {:?}", params);
        let coisa = query_with_params.clone().to_string();
        println!("{}",coisa);
        let rows_affected = sql_query(coisa)
            .execute(connection)
            .map_err(DatabaseError::QueryExecutionError)?;

        println!(
            "Query executada com sucesso! Linhas afetadas: {}",
            rows_affected
        );

        Ok(rows_affected)

        // let mut sql_query = sql_query(query).into_boxed();
        // for param in params {
        //     sql_query = sql_query.bind::<diesel::sql_types::Text, _>(param);
        // }
        

        // let rows_affected = sql_query
        //     .execute(connection)
        //     .map_err(DatabaseError::QueryExecutionError)?;
    
        // println!(
        //     "Query executada com sucesso! Linhas afetadas: {}",
        //     rows_affected
        // );
    
        // Ok(rows_affected)
    }
    

    
}

impl Display for Database {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match &self.connection {
            Some(_) => write!(f, "Conectado ao banco de dados"),
            None => write!(f, "Não há conexão com o banco de dados"),
        }
    }
}

lazy_static! {
    pub static ref SINGLETON_INSTANCE: Mutex<Database> = Mutex::new(Database::new());
}
#[allow(dead_code)]
pub fn get_shared_connection() -> Result<MutexGuard<'static, Database>, DatabaseError> {
    let mut db_instance = SINGLETON_INSTANCE
        .lock()
        .expect("Falha ao obter o lock do singleton");
    db_instance.get_connection()?;
    Ok(db_instance)
}
