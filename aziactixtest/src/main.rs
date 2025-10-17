use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web};
use serde::Serialize;
use serde_json::Value;

#[derive(Serialize)]
struct SimpleSlackResponse {
    response_type: String,
    text: String,
}

fn ephemeral_response(message: String) -> SimpleSlackResponse {
    SimpleSlackResponse {
        response_type: "ephemeral".to_string(),
        text: message,
    }
}

fn in_channel_response(message: String) -> SimpleSlackResponse {
    SimpleSlackResponse {
        response_type: "in_channel".to_string(),
        text: message,
    }
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().json(in_channel_response("Dia dhuit!\n".to_string()))
}

#[post("/")]
async fn submit(info: web::Bytes) -> impl Responder {
    match serde_json::from_slice::<Value>(&info) {
        Ok(json_value) => {
            let response = ephemeral_response(json_value.to_string());
            HttpResponse::Ok().json(response)
        }
        Err(e) => HttpResponse::BadRequest().body(format!("Failed to parse Json: {}", e)),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index).service(submit))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
