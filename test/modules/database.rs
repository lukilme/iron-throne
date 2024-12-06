mod lib_module {
    include!("../../modules/database/connection.rs");
}

#[cfg(test)]
pub mod database_tests {
    use diesel::sql_query;
    use diesel::PgConnection;
    use diesel::QueryableByName;
    use diesel::RunQueryDsl;
    use diesel::{connection::SimpleConnection, QueryResult};
    use dotenvy::dotenv;
    use std::env;
    #[derive(Debug, QueryableByName)]
    struct Test {
        #[diesel(sql_type = diesel::sql_types::Integer)]
        id: i32,
        #[diesel(sql_type = diesel::sql_types::Text)]
        name: String,
    }

    use super::lib_module;

    #[test]
    pub fn test_establish_connection() {
        dotenv().ok();

        let _database_url = env::var("DATABASE_URL").expect("DATABASE_URL deve ser definido");

        let mut diesel_connection = lib_module::establish_connection();

        let result = diesel_connection.batch_execute("SELECT 1;");

        assert!(
            result.is_ok(),
            "Falha ao validar a conexão com o banco de dados"
        );
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

    #[derive(Debug, QueryableByName)]
    struct PgTable {
        #[diesel(sql_type = diesel::sql_types::Text)]
        schemaname: String,
        #[diesel(sql_type = diesel::sql_types::Text)]
        tablename: String,
    }

    pub fn insert_into_test(conn: &mut PgConnection, id: i32, name: &str) -> QueryResult<usize> {
        sql_query("INSERT INTO Test (id, name) VALUES ($1, $2);")
            .bind::<diesel::sql_types::Integer, _>(id)
            .bind::<diesel::sql_types::Text, _>(name)
            .execute(conn)
    }

    pub fn query_test(conn: &mut PgConnection) -> QueryResult<Vec<Test>> {
        sql_query("SELECT id, name FROM Test;").load::<Test>(conn)
    }

    #[test]
    pub fn sql_query_test() {
        use diesel::prelude::*;
        use diesel::sql_query;

        let mut diesel_connection = lib_module::establish_connection();

        let results: QueryResult<Vec<PgTable>> =
            sql_query("SELECT schemaname, tablename FROM pg_catalog.pg_tables;")
                .load(&mut diesel_connection);

        match results {
            Ok(tables) => {
                for table in tables {
                    println!("Schema: {}, Table: {}", table.schemaname, table.tablename);
                }
            }
            Err(e) => eprintln!("Error fetching tables: {}", e),
        }
    }

    #[test]
    pub fn create_table() {
        use diesel::prelude::*;
        use diesel::sql_query;

        #[derive(Debug, QueryableByName)]
        struct Test {
            #[diesel(sql_type = diesel::sql_types::Integer)]
            id: i32,
            #[diesel(sql_type = diesel::sql_types::Text)]
            name: String,
        }

        let mut diesel_connection = lib_module::establish_connection();

        // Criar a tabela
        let create_table_result = sql_query(
            "CREATE TABLE IF NOT EXISTS Test (
                id INT PRIMARY KEY,
                name VARCHAR(16) NOT NULL
            );
            ",
        )
        .execute(&mut diesel_connection);

        assert!(
            create_table_result.is_ok(),
            "Failed to create table: {:?}",
            create_table_result.err()
        );

        // Consultar a tabela criada
        let results: QueryResult<Vec<Test>> =
            sql_query("SELECT * FROM Test;").load(&mut diesel_connection);

        match results {
            Ok(records) => {
                println!("Table data: {:?}", records);
            }
            Err(e) => eprintln!("Error fetching data: {}", e),
        }
    }
    #[test]
    fn test_insert_and_query() {
        let mut conn = lib_module::establish_connection();

        let _ = sql_query(
            "CREATE TABLE IF NOT EXISTS Test (
            id INT PRIMARY KEY,
            name VARCHAR(16) NOT NULL
        );
        ",
        )
        .execute(&mut conn);

        let insert_result = insert_into_test(&mut conn, 1, "Alice");
        assert!(
            insert_result.is_ok(),
            "Failed to insert into Test: {:?}",
            insert_result.err()
        );

        let query_result = query_test(&mut conn);
        assert!(
            query_result.is_ok(),
            "Failed to query Test: {:?}",
            query_result.err()
        );

        let records = query_result.unwrap();
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].id, 1);
        assert_eq!(records[0].name, "Alice");
        println!("Query result: {:?}", records);
    }
}
