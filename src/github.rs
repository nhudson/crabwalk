use actix_web::{post, web, HttpRequest, HttpResponse, Responder, http::header::HeaderMap};
use futures::StreamExt;
use ring::hmac::{self, verify};
use log::error;
use octocrab::models::events::payload::EventPayload;

use crate::config::Config;

#[post("/github")]
pub async fn github(
    config: web::Data<Config>,
    req: HttpRequest,
    mut payload: web::Payload,
) -> impl Responder {

    // Extract body bytes from the payload stream
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        body.extend_from_slice(&chunk.unwrap());
    }
    let body_vec = body.to_vec();

    // Check for content type header is application/json
    if req.headers().get("content-type").unwrap() != "application/json" {
        error!("Invalid content-type in request header");
        return HttpResponse::BadRequest().body("Invalid content-type");
    }

    // Check for X-Hub-Signature-256 header
    let signature = match check_and_generate_signature(req.headers()) {
        Ok(signature) => signature,
        Err(e) => {
            error!("{}", e);
            return HttpResponse::BadRequest().body(e);
        }
    };

    // Verify HMAC signature
    let secret = &config.webhook_secret;
    let key = hmac::Key::new(hmac::HMAC_SHA256, secret.as_bytes());

    // Call verify_payload to verify the signature
    if !verify_payload(&key, &body_vec, &signature) {
        error!("Invalid signature");
        return HttpResponse::Unauthorized().body("Invalid signature");
    }

    // Trust the payload if the signature is valid
    // (TODO): Eventually this will do something with the payload
    let _payload: EventPayload = match serde_json::from_slice(&body_vec) {
        Ok(payload) => payload,
        Err(_) => {
            error!("Invalid payload");
            return HttpResponse::BadRequest().body("Invalid payload");
        }
    };
    HttpResponse::Ok().finish()
}

fn verify_payload(key: &hmac::Key, body: &[u8], signature: &[u8]) -> bool {
    verify(key, body, signature).is_ok()
}

fn check_and_generate_signature(headers: &HeaderMap) -> Result<Vec<u8>, &'static str> {
    let signature = headers.get("X-Hub-Signature-256").ok_or("Missing X-Hub-Signature-256 header")?;
    let signature_str = signature.to_str().map_err(|_| "Invalid X-Hub-Signature-256 header")?;
    let signature_bytes = signature_str.get(7..).ok_or("Invalid signature format")?;
    hex::decode(signature_bytes).map_err(|_| "Invalid signature format")
}
