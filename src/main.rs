use std::net::TcpListener;
use zero2prod::run;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind at random port");
    // Retrieve the port assigned to us by OS
    let port = listener.local_addr().unwrap().port();
    println!("Server running at PORT {}", port);
    run(listener)?.await
}