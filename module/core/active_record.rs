

#[derive(Debug)] 
pub enum DbError {
    NotFound,
    ConnectionError(String),
    Other(String),
}

pub trait ActiveRecord {
    fn save(&self) -> Result<(), DbError>;
    fn delete(&self) -> Result<(), DbError>;
    fn find(id: i64) -> Result<Self, DbError> where Self: Sized;
}