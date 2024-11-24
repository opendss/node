pub trait Stateful {
    async fn start(&mut self);
    async fn close(&mut self);
}