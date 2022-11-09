use reqwest::Client;
use secrecy::{ExposeSecret, Secret};
use serde::ser::SerializeStruct;
use serde::Serialize;

use crate::configuration::settings::EmailSettings;
use crate::domain::model::user_email::UserEmail;

pub struct MailjetClient {
    sender: String,
    base_url: String,
    http_client: Client,
    api_key: Secret<String>,
    secret_key: Secret<String>,
}

impl MailjetClient {
    pub fn new(config: EmailSettings) -> Self {
        let timeout = std::time::Duration::from_secs(1);

        Self {
            http_client: Client::builder().timeout(timeout).build().unwrap(),
            sender: config.sender_email,
            base_url: config.base_url,
            api_key: config.api_key,
            secret_key: config.secret_key,
        }
    }

    pub async fn send_email(
        &self,
        recipient_email: &UserEmail,
        subject: &str,
        html_content: &str,
        text_content: &str,
    ) -> Result<(), reqwest::Error> {
        tracing::info!("Start sending an email");
        let url = format!("{}/send", self.base_url);
        let request_body = SendEmailRequest {
            from_email: self.sender.as_ref(),
            to: recipient_email.as_ref(),
            subject,
            html_part: html_content,
            text_part: text_content,
        };

        self.http_client
            .post(&url)
            .basic_auth(
                self.api_key.expose_secret(),
                Some(&self.secret_key.expose_secret()),
            )
            .json(&request_body)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }
}

struct SendEmailRequest<'a> {
    from_email: &'a str,
    subject: &'a str,
    text_part: &'a str,
    html_part: &'a str,
    to: &'a str,
}

impl<'a> Serialize for SendEmailRequest<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("SendEmailRequest", 5)?;

        s.serialize_field("FromEmail", &self.from_email)?;
        s.serialize_field("Subject", &self.subject)?;
        s.serialize_field("Html-part", &self.html_part)?;
        s.serialize_field("Text-part", &self.text_part)?;
        s.serialize_field("To", &self.to)?;

        s.end()
    }
}

#[cfg(test)]
mod tests {
    use claim::{assert_err, assert_ok};
    use fake::faker::internet::en::SafeEmail;
    use fake::faker::lorem::en::{Paragraph, Sentence};
    use fake::{Fake, Faker};
    use secrecy::Secret;
    use wiremock::matchers::{header, header_exists, method, path};
    use wiremock::{Mock, MockServer, Request, ResponseTemplate};

    use crate::configuration::settings::EmailSettings;
    use crate::domain::model::user_email::UserEmail;
    use crate::provider::email::mailjet::MailjetClient;

    struct SendEmailBodyMatcher;

    impl wiremock::Match for SendEmailBodyMatcher {
        fn matches(&self, request: &Request) -> bool {
            let result: Result<serde_json::Value, _> = serde_json::from_slice(&request.body);

            if let Ok(body) = result {
                body.get("FromEmail").is_some()
                    && body.get("To").is_some()
                    && body.get("Subject").is_some()
                    && body.get("Html-part").is_some()
                    && body.get("Text-part").is_some()
            } else {
                false
            }
        }
    }

    #[actix_web::test]
    async fn send_email_sends_the_expected_request() {
        let mock_server = MockServer::start().await;

        create_mock(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        let email_client = init_email_client(mock_server.uri());
        let result = send_email_request(&email_client).await;

        assert_ok!(result);
    }

    #[actix_web::test]
    async fn send_email_fails_if_the_api_returns_500() {
        let mock_server = MockServer::start().await;

        create_mock(ResponseTemplate::new(500))
            .mount(&mock_server)
            .await;

        let email_client = init_email_client(mock_server.uri());
        let result = send_email_request(&email_client).await;

        assert_err!(result);
    }

    #[actix_web::test]
    async fn send_email_times_out_if_the_api_takes_too_long() {
        let mock_server = MockServer::start().await;

        let mock_response =
            ResponseTemplate::new(200).set_delay(std::time::Duration::from_secs(180));

        create_mock(mock_response).mount(&mock_server).await;

        let email_client = init_email_client(mock_server.uri());
        let result = send_email_request(&email_client).await;

        assert_err!(result);
    }

    fn create_mock(response_template: ResponseTemplate) -> Mock {
        Mock::given(header_exists("Authorization"))
            .and(header("Content-Type", "application/json"))
            .and(path("/send"))
            .and(method("POST"))
            .and(SendEmailBodyMatcher)
            .respond_with(response_template)
            .expect(1)
    }

    fn init_email_client(base_url: String) -> MailjetClient {
        let sender_email = SafeEmail().fake();

        MailjetClient::new(EmailSettings {
            base_url,
            sender_email,
            api_key: Secret::new(Faker.fake()),
            secret_key: Secret::new(Faker.fake()),
        })
    }

    async fn send_email_request(email_client: &MailjetClient) -> Result<(), reqwest::Error> {
        let subscriber_email = UserEmail::parse(SafeEmail().fake()).unwrap();
        let subject: String = Sentence(1..2).fake();
        let content: String = Paragraph(1..10).fake();

        email_client
            .send_email(&subscriber_email, &subject, &content, &content)
            .await
    }
}
