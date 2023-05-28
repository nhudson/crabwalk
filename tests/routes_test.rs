#[cfg(test)]
mod test {
    use actix_web::{http::header::ContentType, http::StatusCode, test, web, App};
    use crabwalk::{config::Config, github::github as gh, health::liveness, health::readiness};
    use ring::hmac;

    #[actix_web::test]
    #[ignore]
    async fn test_github_route_200() {
        // Build out Config struct where slack_token, webhook_secret, and github_token are not empty
        let config = Config {
            webhook_secret: "7b73b7c3b0fde4f79f6e80a95c9dab06a4289b591f3b0913cda355f50e595a19"
                .to_string(),
            github_token: "1234ABCD".to_string(),
            slack_token: "1234ABCD".to_string(),
            server_host: "0.0.0.0".to_string(),
            server_port: 8080,
            log_level: "debug".to_string(),
        };
        let config_data = web::Data::new(config.clone());

        // Start webserver with github route
        let app = test::init_service(App::new().app_data(config_data).service(gh)).await;

        // load test payload.json from testsdata directory
        let payload = include_str!("../testdata/payload.json").as_bytes();

        // Generate a HMAC-SHA256 hash using the secret key and the payload
        let key = hmac::Key::new(hmac::HMAC_SHA256, config.webhook_secret.as_bytes());
        let signature = hmac::sign(&key, payload);

        // Build POST request with payload and signature
        let req = test::TestRequest::post()
            .uri("/github")
            .insert_header(ContentType::json())
            .insert_header((
                "X-Hub-Signature-256",
                format!("sha256={}", hex::encode(signature)),
            ))
            .set_payload(payload)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    #[ignore]
    async fn test_liveness_probe_200() {
        // Build out Config struct where slack_token, webhook_secret, and github_token are not empty
        let config = Config {
            webhook_secret: "7b73b7c3b0fde4f79f6e80a95c9dab06a4289b591f3b0913cda355f50e595a19"
                .to_string(),
            github_token: "1234ABCD".to_string(),
            slack_token: "1234ABCD".to_string(),
            server_host: "0.0.0.0".to_string(),
            server_port: 8080,
            log_level: "debug".to_string(),
        };
        let config_data = web::Data::new(config.clone());

        // Start webserver with liveness route
        let app = test::init_service(App::new().app_data(config_data).service(liveness)).await;

        // Make sure app is running by querying the liveness route
        let req = test::TestRequest::get()
            .uri("/health/liveness")
            .to_request();

        // Check for 200 OK back from request
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);

        // Check for "I'm alive!" in response body
        let body = test::read_body(resp).await;
        assert_eq!(&body[..], r#""I'm alive!""#.as_bytes());
    }

    #[actix_web::test]
    #[ignore]
    async fn test_readiness_probe_200() {
        // Build out Config struct where slack_token, webhook_secret, and github_token are not empty
        let config = Config {
            webhook_secret: "7b73b7c3b0fde4f79f6e80a95c9dab06a4289b591f3b0913cda355f50e595a19"
                .to_string(),
            github_token: "1234ABCD".to_string(),
            slack_token: "1234ABCD".to_string(),
            server_host: "0.0.0.0".to_string(),
            server_port: 8080,
            log_level: "debug".to_string(),
        };
        let config_data = web::Data::new(config.clone());

        // Start webserver with liveness route
        let app = test::init_service(App::new().app_data(config_data).service(readiness)).await;

        // Make sure app is running by querying the liveness route
        let req = test::TestRequest::get()
            .uri("/health/readiness")
            .to_request();

        // Check for 200 OK back from request
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);

        // Check for "I'm ready!" in response body
        let body = test::read_body(resp).await;
        assert_eq!(&body[..], r#""I'm ready!""#.as_bytes());
    }
}
