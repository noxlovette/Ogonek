use aws_config::{BehaviorVersion, Region};
use aws_credential_types::Credentials;
use aws_sdk_s3::Client as S3Client;

pub async fn init_s3() -> anyhow::Result<S3Client> {
    let s3_config = aws_config::defaults(BehaviorVersion::latest())
    .region(Region::new(std::env::var("SCW_REGION").expect("SCW_REGIONmust be sset"))) // 
    .endpoint_url(std::env::var("SCW_URL").expect("SCW_URL must be set")) 
    .credentials_provider(Credentials::new(
        std::env::var("SCW_ACCESS_KEY").expect("SCW_ACCESS_KEY must be set"),
        std::env::var("SCW_SECRET_KEY").expect("SCW_SECRET_KEY must be set"),
        None, None, "scaleway-credentials"
    ))
    .load()
    .await;

    let s3_client = S3Client::new(&s3_config);

    Ok(s3_client)
}


pub async fn init_s3_new() -> anyhow::Result<S3Client> {

    let region = std::env::var("SCW_REGION")
        .map_err(|_| anyhow::anyhow!("SCW_REGION environment variable is missing"))?;
    let endpoint = std::env::var("SCW_URL")
        .map_err(|_| anyhow::anyhow!("SCW_URL environment variable is missing"))?;
    let access_key = std::env::var("SCW_ACCESS_KEY")
        .map_err(|_| anyhow::anyhow!("SCW_ACCESS_KEY environment variable is missing"))?;
    let secret_key = std::env::var("SCW_SECRET_KEY")
        .map_err(|_| anyhow::anyhow!("SCW_SECRET_KEY environment variable is missing"))?;

    tracing::debug!("S3 Configuration: Region={}, Endpoint={}", region, endpoint);

    let credentials = Credentials::new(
        access_key,
        secret_key,
        None, 
        None, 
        "scaleway-credentials"
    );

    // Build the config with explicit timeout
    let s3_config = aws_config::defaults(BehaviorVersion::latest())
        .region(Region::new(region))
        .endpoint_url(endpoint)
        .credentials_provider(credentials)
        .timeout_config(
            aws_sdk_s3::config::timeout::TimeoutConfig::builder()
                .connect_timeout(std::time::Duration::from_secs(10))
                .operation_timeout(std::time::Duration::from_secs(30))
                .build()
        )
        .load()
        .await;

    let s3_client = S3Client::new(&s3_config);
    
    match s3_client.list_buckets().send().await {
        Ok(_) => tracing::info!("Successfully connected to S3 service"),
        Err(e) => tracing::warn!("Could not verify S3 connection: {}", e),
    }

    Ok(s3_client)
}