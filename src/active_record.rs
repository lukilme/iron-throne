use crate::erro::DbError;

pub trait ActiveRecord {
    fn save(&self) -> Result<(), DbError>;
}
