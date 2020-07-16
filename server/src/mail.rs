// extern crate lettre;
// extern crate lettre_email;
use lettre::smtp::authentication::IntoCredentials;
use lettre::{SmtpClient, Transport};
use lettre_email::EmailBuilder;
use std::env;

static USERNAME: &str = "anomaly.detector2020@gmail.com";
static PASSWORD: &str = env!("PASSWORD","Missing password ENV");
static SMTP_ADDRESS: &str = "smtp.gmail.com";

pub fn send_mail(to: &str,subject: &str,text: &str) -> bool {
    let mut email = EmailBuilder::new()
        .to(to)
        .from(USERNAME)
        .subject(subject)
        .text(text)
        .build();
       let email = match email {
          Ok(msg) => msg.into(),
          Err(e) => return false
        };
    let credentials = (USERNAME, PASSWORD).into_credentials();
    let mut client = SmtpClient::new_simple(SMTP_ADDRESS)
        .unwrap()
        .credentials(credentials)
        .transport();
    client.send(email).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn send_mail_test() {
        assert_eq!(send_mail("fineyoc131@djemail.net","empty subject","Since this is a simple and common operation, it would be convenient if it could be elided. Alas, because and_then is not sufficiently flexible, it cannot. However, we can instead use ?."),true);
    }

    #[test]
    fn send_mail_expect_false() {
        assert_eq!(send_mail("wrong.email.net","empty subject","Since this is a simple and common operation, it would be convenient if it could be elided. Alas, because and_then is not sufficiently flexible, it cannot. However, we can instead use ?."),false);
    }
}
