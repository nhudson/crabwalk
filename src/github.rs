use actix_web::{post, web, HttpRequest, HttpResponse, Responder};
use futures::StreamExt;
use hex::decode;
use hmac::{Hmac, Mac};
use log::error;
use sha1::Sha1;
use subtle::ConstantTimeEq;
//use slack_hook::{PayloadBuilder, Slack};

use crate::config::Config;

#[post("/github")]
pub async fn github(
    config: web::Data<Config>,
    req: HttpRequest,
    mut payload: web::Payload,
) -> impl Responder {
    // Extract body bytes from the Payload stream
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        body.extend_from_slice(&chunk.unwrap());
    }

    // Extract the X-Hub-Signature header from the request
    let signature = req
        .headers()
        .get("X-Hub-Signature")
        .and_then(|hv| hv.to_str().ok())
        .unwrap_or_default()
        .to_string();

    if verify_webhook_token(&config.webhook_secret, &signature) {
        HttpResponse::Ok().finish()
    } else {
        error!("Invalid signature from /github request");
        HttpResponse::Unauthorized().body("Invalid signature")
    }
}

fn verify_webhook_token(secret: &str, signature: &str) -> bool {
    // Github signature is in sha1=<hash> format
    if !signature.starts_with("sha1=") {
        return false;
    }

    // Remove prefix and decode hex
    let signature_bytes = match decode(&signature[5..]) {
        Ok(bytes) => bytes,
        Err(_) => return false,
    };

    // Create HMAC-SHA1 signature of the payload with our secret key
    let mac = Hmac::<Sha1>::new_from_slice(secret.as_bytes()).expect("Failed to create HMAC");
    let expected_signature = mac.finalize().into_bytes();

    // use constant_time::constant_time_eq to compare the two signatures
    let signature_eq = signature_bytes.ct_eq(&expected_signature);
    signature_eq.into()
}
