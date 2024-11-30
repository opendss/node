use crate::common::errors::CommonError;


pub trait Stateful {
    async fn start(&mut self) -> Result<(), CommonError>;
    async fn close(&mut self) -> Result<(), CommonError>;
}
