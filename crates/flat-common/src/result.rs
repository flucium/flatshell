use crate::error;

pub type Result<T> = core::result::Result<T, error::Error>;
