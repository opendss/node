use thiserror::Error;

#[derive(Error, Debug)]
pub enum CommonError {
    #[error("io error")]
    IO(#[from] std::io::Error),
}
