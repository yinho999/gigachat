use std::net::TcpListener;
use actix_web::rt::spawn;

pub struct TestApp {
    pub address: String,
    pub port: u16,
    pub api_client: reqwest::Client,
}

pub async fn spawn_app() -> TestApp {
    // Bind to port 0
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to port");
    // Get address
    let address = format!("http://127.0.0.1:{}", listener.local_addr().unwrap().port());
    // Get the port
    let port = listener.local_addr().unwrap().port();
    // Build the application
    let server = gigachat_backend::startup::run(listener).await.expect("Failed to bind address");
    let _ = spawn(server);
    // Create a reqwest client
    let api_client = reqwest::Client::builder()
        .build().unwrap();
    TestApp {
        address,
        port,
        api_client,
    }
}

