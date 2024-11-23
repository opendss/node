pub trait Stateful {
    async fn start();
    fn close();
}