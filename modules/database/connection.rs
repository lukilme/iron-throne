use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::env;
use dotenvy::dotenv;


pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL deve ser definido");

    PgConnection::establish(&database_url).expect("Erro ao conectar ao banco de dados")
}
