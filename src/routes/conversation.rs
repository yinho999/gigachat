use std::convert::Infallible;
use std::fmt::Formatter;
use actix_web::error::InternalError;
use actix_web::{HttpResponse, web};
use actix_web::web::Data;
use llm::{ Model};
use llm::InferenceFeedback::Halt;
use llm::models::Llama;
use crate::models::{Conversation, User};

#[derive(thiserror::Error)]
pub enum ChatError{
    #[error("Inference Error {0}")]
    InferenceError(#[source] anyhow::Error),
    #[error("Something went wrong")]
    UnexpectedError(#[from] anyhow::Error),
}
impl std::fmt::Debug for ChatError{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}
pub fn error_chain_fmt(
    e: &impl std::error::Error,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    // Display the error
    writeln!(f, "{}\n", e)?;
    // Display the source of the error, if any
    let mut current = e.source();
    while let Some(cause) = current {
        writeln!(f, "Caused by:\n\t{}", cause)?;
        current = cause.source();
    }
    Ok(())
}

fn inference_callback<'a>(stop_sequence: String, buf: &'a mut String, output_str: &'a mut String) -> impl FnMut(llm::InferenceResponse) -> Result<llm::InferenceFeedback, Infallible> + 'a {
    move |req| -> Result<llm::InferenceFeedback, Infallible>{
        match req {
            llm::InferenceResponse::InferredToken(t) => {
                let mut reverse_buf = buf.clone();
                reverse_buf.push_str(&t);
                if stop_sequence.as_str().eq(&reverse_buf) {
                    buf.clear();
                    return Ok::<llm::InferenceFeedback, Infallible>(Halt);
                } else if stop_sequence.as_str().starts_with(&reverse_buf) {
                    buf.push_str(&t);
                    return Ok::<llm::InferenceFeedback, Infallible>(llm::InferenceFeedback::Continue);
                }
                if buf.is_empty() {
                    output_str.push_str(&t);
                } else {
                    output_str.push_str(&reverse_buf);
                }
                Ok::<llm::InferenceFeedback, Infallible>(llm::InferenceFeedback::Continue)
            }
            llm::InferenceResponse::EotToken => Ok(Halt),
            _ => Ok(llm::InferenceFeedback::Continue),
        }
    }
}

pub async fn conversation(data: Data<Llama>, prompt: web::Json<Conversation>) -> Result<HttpResponse, InternalError<ChatError>> {
    println!("Conversation: {:?}", prompt);
    let model = data.into_inner();
    let character_name = "### Assistant";
    let user_name = "### User";
    let persona = &prompt.persona; // "A chat between a human and an assistant.";
    let mut chat = format!(
        "{character_name}: Hello - How may I help you today?\n\
        {user_name}: What is the capital of France.\n\
        {character_name}: Paris is the capital of France.\n");
    for message in &prompt.messages {
        let user = match message.user {
            User::Human => user_name,
            User::Bot => character_name,
        };
        let chat_message = &format!("{user}: {text}\n", user = user, text = message.text);
        chat.push_str(chat_message);
    }
    let mut res = String::new();
    let mut rng = rand::thread_rng();
    let mut buffer = String::new();
    let mut session = model.start_session(Default::default());
    session.infer(
        // this is the model we want to use
        model.as_ref(),
        // this is the rng we want to use
        &mut rng,
        // this is the request with settings we want to make
        &llm::InferenceRequest {
            // this is the input prompt
            prompt: format!("{}\n{}\n{}:", persona, chat, character_name).as_str().into(),
            // the parameters for the inference
            parameters: &llm::InferenceParameters::default(),
            // does the response include the input prompt?
            play_back_previous_tokens: false,
            // the maximum number of tokens to generate
            maximum_token_count: None,
        },
        &mut Default::default(),
        // This is the callback function that will be called when the model needs to end the conversation
        inference_callback(String::from(user_name), &mut res, &mut buffer),
    ).map_err(|e| InternalError::new(ChatError::InferenceError(e.into()), actix_web::http::StatusCode::INTERNAL_SERVER_ERROR))?;


    Ok(HttpResponse::Ok().json(buffer))
}