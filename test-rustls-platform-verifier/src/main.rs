#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder()
        .use_preconfigured_tls(rustls_platform_verifier::tls_config())
        .build()
        .expect("Build should not fail");

    let resp = client
        .get("https://httpbin.org/ip")
        .send()
        .await?
        .text()
        .await?;

    println!("body = {resp:?}");

    Ok(())
}
