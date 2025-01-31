use crate::erro::DbError;
//use tokio_postgres::{Client, NoTls};

pub fn execute_query(query: &str) -> Result<(), DbError> {
    //let connection_string = "host=localhost user=postgres dbname=iron_throne password=postgres";
    println!("{}",query);
    // let (client, connection) = tokio_postgres::connect(connection_string, NoTls)
    //     .await.map_err(|e| DbError {
    //         message: format!("Failed to connect to database: {}", e),
    //     })?;

    // // Executa a consulta
    // client
    //     .execute(query, &[])
    //     .map_err(|e| DbError {
    //         message: format!("Failed to execute query: {}", e),
    //     })?;

    Ok(())
}