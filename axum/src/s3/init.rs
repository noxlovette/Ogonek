use aws_config::{BehaviorVersion, Region};
use aws_credential_types::Credentials;
use aws_sdk_s3::Client as S3Client;

pub async fn init_s3() -> anyhow::Result<S3Client> {
    let region = std::env::var("S3_REGION")
        .map_err(|_| anyhow::anyhow!("S3_REGION environment variable is missing"))?;
    let endpoint = std::env::var("S3_URL")
        .map_err(|_| anyhow::anyhow!("S3_URL environment variable is missing"))?;
    let access_key = std::env::var("S3_ACCESS_KEY")
        .map_err(|_| anyhow::anyhow!("S3_ACCESS_KEY environment variable is missing"))?;
    let secret_key = std::env::var("S3_SECRET_KEY")
        .map_err(|_| anyhow::anyhow!("S3_SECRET_KEY environment variable is missing"))?;

    tracing::debug!("S3 Configuration: Region={}, Endpoint={}", region, endpoint);

    let credentials = Credentials::new(access_key, secret_key, None, None, "scaleway-credentials");

    let s3_config = aws_config::defaults(BehaviorVersion::latest())
        .region(Region::new(region))
        .endpoint_url(endpoint)
        .credentials_provider(credentials)
        .timeout_config(
            aws_sdk_s3::config::timeout::TimeoutConfig::builder()
                .connect_timeout(std::time::Duration::from_secs(10))
                .operation_timeout(std::time::Duration::from_secs(30))
                .build(),
        )
        .load()
        .await;

    let s3_client = S3Client::new(&s3_config);

    // instantly check if the connection is alive
    match s3_client.list_buckets().send().await {
        Ok(_) => tracing::info!("Successfully connected to S3 service"),
        Err(e) => tracing::warn!("Could not verify S3 connection: {}", e),
    }

    Ok(s3_client)
}
