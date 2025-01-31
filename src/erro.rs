#[derive(Debug)]
pub struct DbError {
    pub message: String,
}
impl std::fmt::Display for DbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Database error: {}", self.message)
    }
}

impl From<tokio_postgres::Error> for DbError {
    fn from(error: tokio_postgres::Error) -> Self {
        DbError {
            message: error.to_string(),
        }
    }
}