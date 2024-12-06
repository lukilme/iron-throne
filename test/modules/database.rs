mod lib_module {
    include!("../../modules/database/connection.rs");
}

#[cfg(test)]
pub mod database_tests {
    use diesel::{connection::SimpleConnection, QueryResult}; 
    use dotenvy::dotenv;
    use std::env;

    use super::lib_module;

    #[test]
    pub fn test_establish_connection() {
        dotenv().ok();

        let _database_url = env::var("DATABASE_URL").expect("DATABASE_URL deve ser definido");
     
        let mut diesel_connection = lib_module::establish_connection();

        let result = diesel_connection.batch_execute("SELECT 1;");

        assert!(result.is_ok(), "Falha ao validar a conexão com o banco de dados");
    }

    #[test]
    pub fn show_tables() {
    
    
        let mut diesel_connection = lib_module::establish_connection();
    
        let result = diesel_connection.batch_execute("SELECT * FROM pg_catalog.pg_tables;");
    
        match result {
            Ok(_) => println!("Command executed successfully."),
            Err(e) => eprintln!("Error executing command: {}", e),
        }
    }
    
    #[test]
    pub fn sql_query_test() {
        use diesel::RunQueryDsl;
        use diesel::sql_query;
        use diesel::QueryableByName;

        #[derive(Debug, QueryableByName)]
        struct TableName {
            #[diesel(sql_type = diesel::sql_types::Text)]
            table_name: String,
        }

        let mut diesel_connection = lib_module::establish_connection();

        let results: QueryResult<Vec<TableName>> = sql_query("SELECT * FROM pg_catalog.pg_tables;").load(&mut diesel_connection);

        match results {
            Ok(tables) => {
                for table in tables {
                    println!("Table: {}", table.table_name);
                }
            }
            Err(e) => eprintln!("Error fetching tables: {}", e),
        }
    }

}
