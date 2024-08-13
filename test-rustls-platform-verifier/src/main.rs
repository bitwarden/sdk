#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let client = reqwest::Client::builder()
        .use_preconfigured_tls(rustls_platform_verifier::tls_config())
        .build()
        .expect("Build should not fail");
    let request = client.get("https://httpbin.org/ip").build()?;
    let response = client.execute(request).await?;

    let status_code = response.status();
    let content = response.text().await?;

    println!("status_code = {status_code:?}");
    println!("content = {content:?}");

    Ok(())
}
