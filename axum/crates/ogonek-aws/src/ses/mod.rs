use aws_config::{BehaviorVersion, Region};
use aws_credential_types::Credentials;
use aws_sdk_sesv2::Client as SesClient;

mod emails;
mod error;
mod tera;
pub use error::SESError;

#[derive(Debug, Clone)]
pub struct SESProvider {
    client: SesClient,
    from_email: String,
}

impl SESProvider {
    pub async fn new() -> anyhow::Result<Self> {
        let region = std::env::var("SES_REGION")
            .map_err(|_| anyhow::anyhow!("SES_REGION environment variable is missing"))?;
        let endpoint = std::env::var("SES_URL")
            .map_err(|_| anyhow::anyhow!("SES_URL environment variable is missing"))?;
        let access_key = std::env::var("SES_ACCESS_KEY")
            .map_err(|_| anyhow::anyhow!("SES_ACCESS_KEY environment variable is missing"))?;
        let secret_key = std::env::var("SES_SECRET_KEY")
            .map_err(|_| anyhow::anyhow!("SES_SECRET_KEY environment variable is missing"))?;

        let from_email = std::env::var("SES_EMAIL").expect("SES_EMAIL should be set");

        tracing::debug!(
            "SES Configuration: Region={}, Endpoint={}",
            region,
            endpoint
        );

        let credentials = Credentials::new(access_key, secret_key, None, None, "ses-credentials");

        let ses_config = aws_config::defaults(BehaviorVersion::latest())
            .region(Region::new(region))
            .endpoint_url(endpoint)
            .credentials_provider(credentials)
            .timeout_config(
                aws_sdk_ses::config::timeout::TimeoutConfig::builder()
                    .connect_timeout(std::time::Duration::from_secs(10))
                    .operation_timeout(std::time::Duration::from_secs(30))
                    .build(),
            )
            .load()
            .await;

        let client = SesClient::new(&ses_config);

        Ok(Self { client, from_email })
    }
}
