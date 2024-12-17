use sqlx::postgres::PgPoolOptions;
use sqlx::Error;

#[tokio::test]
async fn main() -> Result<(), Error> {

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://admin:admin@localhost/teste")
        .await?;

    match find_user_by_id(&pool, 1).await {
        Ok(user_name) => println!("Found user: {}", user_name),
        Err(e) => eprintln!("Error: {}", e),
    }

    Ok(())
}

async fn find_user_by_id(pool: &sqlx::PgPool, user_id: i32) -> Result<String, sqlx::Error> {
    let row = sqlx::query!("SELECT name FROM \"Users\" WHERE id = $1", user_id)
        .fetch_one(pool)
        .await?;
    
    Ok(row.name)
}
