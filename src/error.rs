use std::result::Result as GenericResult;

pub type Result<T> = GenericResult<T, AppError>;

#[derive(Debug)]
pub enum AppError {}
