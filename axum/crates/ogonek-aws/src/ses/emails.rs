use crate::ses::tera::TEMPLATES;

use super::{SESError, SESProvider};
use aws_sdk_sesv2::types::*;

impl SESProvider {
    pub async fn send_plaintext_email(
        &self,
        to: &str,
        subject: &str,
        body: &str,
    ) -> Result<(), SESError> {
        self.client
            .send_email()
            .from_email_address(&self.from_email)
            .destination(Destination::builder().to_addresses(to).build())
            .content(
                EmailContent::builder()
                    .simple(
                        Message::builder()
                            .subject(Content::builder().data(subject).build()?)
                            .body(
                                Body::builder()
                                    .text(Content::builder().data(body).build()?)
                                    .build(),
                            )
                            .build(),
                    )
                    .build(),
            )
            .send()
            .await?;

        Ok(())
    }

    pub async fn send_confirm_email(
        &self,
        to: &str,
        name: &str,
        role: &str,
        token: &str,
    ) -> Result<(), SESError> {
        // Just grab the static reference when you need it
        let tera = &TEMPLATES; // or Tera::instance()

        let address = std::env::var("FRONTEND_URL").unwrap_or("http://ogonek.app".to_string());

        let mut ctx = tera::Context::new();
        ctx.insert("name", name);
        ctx.insert("role", role);
        ctx.insert(
            "app_url",
            format!("{}/confirm-email/?token={}", address, token).as_str(),
        );

        let html = tera.render("confirm.html", &ctx)?;

        // Send via SES
        self.client
            .send_email()
            .from_email_address(&self.from_email)
            .destination(Destination::builder().to_addresses(to).build())
            .content(
                EmailContent::builder()
                    .simple(
                        Message::builder()
                            .subject(Content::builder().data("Добро пожаловать").build()?)
                            .body(
                                Body::builder()
                                    .html(Content::builder().data(html).build()?)
                                    .build(),
                            )
                            .build(),
                    )
                    .build(),
            )
            .send()
            .await?;

        tracing::info!("Confirm email sent");
        Ok(())
    }
}
