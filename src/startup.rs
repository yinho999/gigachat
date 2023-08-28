use std::net::TcpListener;
use std::path::PathBuf;
use actix_web::{App, HttpServer, web};
use actix_web::dev::Server;
use llm::{KnownModel, LoadError};
use llm::models::Llama;
use crate::routes::{conversation, health_check};

async fn load_llm_model(path: PathBuf) -> Result<Llama, LoadError> {
    let model = Llama::load(&path, llm::TokenizerSource::Embedded, Default::default(), llm::load_progress_callback_stdout)?;
    Ok(model)
}

pub async fn run(tcp_listener: TcpListener) -> Result<Server, std::io::Error> {
    let model_path = PathBuf::from(format!("/home/{}/.llm-models/TheBloke/Wizard-Vicuna-7B-Uncensored-GGML/Wizard-Vicuna-7B-Uncensored.ggmlv3.q8_0.bin", std::env::var("USER").unwrap()));
    let model = load_llm_model(model_path).await.unwrap_or_else(|e| panic!("Failed to load model: {}", e));
    let model = web::Data::new(model);
    println!("Server running at http://{}", &tcp_listener.local_addr().unwrap());
    let server = HttpServer::new(move || {
        App::new()
            .app_data(model.clone())
            .route("/health_check", web::get().to(health_check))
            .route("/conversation", web::post().to(conversation))
    }).listen(tcp_listener)?.run();
    Ok(server)
}