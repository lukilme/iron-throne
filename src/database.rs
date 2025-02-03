use crate::erro::DbError;
use dotenvy::dotenv;
use lazy_static::lazy_static;
use std::env;
use std::fmt::{self, Display, Formatter};
use tokio_postgres::{Client, NoTls, types::ToSql};

pub struct Database {
    client: Option<Client>,
}

impl Database {
    pub fn new() -> Self {
        Database { client: None }
    }

    pub async fn connect(&mut self) -> Result<(), DbError> {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").map_err(|_| DbError {
            message: "DATABASE_URL environment variable not set".to_string(),
        })?;

        let (client, connection) = tokio_postgres::connect(&database_url, NoTls)
            .await
            .map_err(|e| DbError {
                message: format!("Failed to connect to database: {}", e),
            })?;

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("Connection error: {}", e);
            }
        });

        self.client = Some(client);
        Ok(())
    }

    pub async fn get_client(&mut self) -> Result<&mut Client, DbError> {
        if self.client.is_none() {
            self.connect().await?;
        }
        self.client.as_mut().ok_or_else(|| DbError {
            message: "Database client not initialized".to_string(),
        })
    }

    pub async fn execute_query(&mut self, query: &str) -> Result<u64, DbError> {
        let client = self.get_client().await?;
        println!("{}",query);
        let rows_affected = client
            .execute(query, &[])
            .await
            .map_err(|e| DbError {
                message: format!("Query execution error: {}", e),
            })?;

        println!("Query executed successfully. Rows affected: {}", rows_affected);
        Ok(rows_affected)
    }

    pub async fn execute_query_with_params(
        &mut self,
        query: &str,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<u64, DbError> {
        let client = self.get_client().await?;
        let rows_affected = client
            .execute(query, params)
            .await
            .map_err(|e| DbError {
                message: format!("Query with parameters execution error: {}", e),
            })?;

        println!(
            "Query with parameters executed successfully. Rows affected: {}",
            rows_affected
        );
        Ok(rows_affected)
    }
}

impl Display for Database {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Database connection status: {}",
            if self.client.is_some() {
                "Connected"
            } else {
                "Disconnected"
            }
        )
    }
}

lazy_static! {
    pub static ref SINGLETON_INSTANCE: tokio::sync::Mutex<Database> =
        tokio::sync::Mutex::new(Database::new());
}

pub async fn get_shared_connection() -> Result<tokio::sync::MutexGuard<'static, Database>, DbError> {
    let mut db = SINGLETON_INSTANCE.lock().await;
    db.get_client().await?;
    Ok(db)
}

pub async fn execute_query(query: &str) -> Result<u64, DbError> {
    let mut db = get_shared_connection().await?;
    db.execute_query(query).await
}
//inutil
pub async fn execute_query_with_params(
    query: &str,
    params: &[&(dyn ToSql + Sync)],
) -> Result<u64, DbError> {
    let mut db = get_shared_connection().await?;
    db.execute_query_with_params(query, params).await
}