use std::process::ExitCode;

use reqwest::Client;

#[tokio::main]
async fn main() -> ExitCode {
    let client = Client::new();

    let result = client
        .get("http://localhost/_health")
        .send()
        .await
        .expect("unable to make web request");

    if result.status().is_success() {
        return ExitCode::SUCCESS;
    }

    let reason = result
        .status()
        .canonical_reason()
        .unwrap_or("Unknown error")
        .to_string();

    println!(
        "Healthcheck failed: {} {}",
        result.status().as_u16(),
        reason
    );

    ExitCode::FAILURE
}
