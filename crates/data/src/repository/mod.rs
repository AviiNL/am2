type RepositoryResult<T> = Result<T, Box<dyn std::error::Error>>;

mod user_repository;

pub use user_repository::*;
