pub trait SQLResource {
  async fn execute_query(&self);
  async fn ping(&self);
}