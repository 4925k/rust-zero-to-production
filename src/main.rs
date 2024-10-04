use std::net::TcpListener;
use zero2prod::run;
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Could not listen for connections");

    run(listener)?.await
}
