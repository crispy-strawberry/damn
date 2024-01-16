use damn_core::resolve_connection;

#[tokio::main]
async fn main() {
    resolve_connection("jabber.org").await.unwrap();
    println!("Hello, world!");
}
