pub mod prelude {
    pub use crate::active_record::ActiveRecord;
    pub use crate::erro::DbError;
    pub use macros::ActiveRecord;
}

pub mod active_record {
    pub trait ActiveRecord {
        fn save(&self) -> Result<(), crate::erro::DbError>;
    }
}