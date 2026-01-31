use std::sync::Arc;

use storage::inmemory::InMemoryOtpRepository;
use di::Container;
use ports::httpapi::server::Server;

pub mod storage;
pub mod di;
pub mod ports;
pub mod domain;
pub mod app;

#[tokio::main]
async fn main() {
    let repo = Arc::new(InMemoryOtpRepository::new());
    let container = Arc::new(Container::new(repo));

    let server = Server::new(8080, container);
    println!("Server starting...");
    server.run().await;
    println!("Server shutted down");
}
