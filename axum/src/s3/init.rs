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
