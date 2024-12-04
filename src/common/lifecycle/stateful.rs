use crate::common::errors::CommonError;
use tonic::async_trait;

#[async_trait]
pub trait Stateful {
    async fn start(&mut self) -> Result<(), CommonError>;
    async fn close(&mut self) -> Result<(), CommonError>;
}
