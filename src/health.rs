use actix_web::{get, HttpRequest, HttpResponse, Responder};
use reqwest;

#[get("/health/liveness")]
pub async fn liveness(_: HttpRequest) -> impl Responder {
    HttpResponse::Ok().json("I'm alive!")
}

#[get("/health/readiness")]
pub async fn readiness(_: HttpRequest) -> impl Responder {
    // Example check: can we reach the GitHub API?
    let client = reqwest::Client::new();
    let response = client
        .get("https://api.github.com")
        .header(reqwest::header::USER_AGENT, "crabwalk")
        .send()
        .await;
    match response {
        Ok(res) if res.status().is_success() => HttpResponse::Ok().json("I'm ready!"),
        _ => HttpResponse::ServiceUnavailable().json("Service unavailable"),
    }
}
