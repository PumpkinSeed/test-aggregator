use lettre::smtp::authentication::IntoCredentials;
use lettre::{SmtpClient, Transport};
use lettre_email::EmailBuilder;
use std::env;
use std::fmt;

static MAILER_USERNAME: &str = "anomaly.detector2020@gmail.com";
static MAILER_PASSWORD: &str = env!("MAILER_PASSWORD","Missing password ENV");
static SMTP_ADDRESS: &str = "smtp.gmail.com";

#[derive(Debug, Clone)]
pub struct MailError;

impl fmt::Display for MailError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "Unable to send email")
  }
}

pub fn send_mail(to: &str,subject: &str,text: &str) -> Result<(),MailError> {
    let mut email = EmailBuilder::new()
        .to(to)
        .from(MAILER_USERNAME)
        .subject(subject)
        .text(text)
        .build();
    let email = match email {
          Ok(msg) => msg.into(),
          Err(e) => {
                    eprintln!("{}",e);
                    return Err(MailError)
                    }
        };
    let credentials = (MAILER_USERNAME, MAILER_PASSWORD).into_credentials();
    let mut client = SmtpClient::new_simple(SMTP_ADDRESS)
        .unwrap()
        .credentials(credentials)
        .transport();
    match client.send(email).is_ok() {
      true => return Ok(()),
      false =>  return Err(MailError)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn send_mail_test() {
        send_mail("fineyoc131@djemail.net","empty subject","Since this is a simple and common operation, it would be convenient if it could be elided. Alas, because and_then is not sufficiently flexible, it cannot. However, we can instead use ?.").unwrap();
    }

    #[test]
    fn send_mail_error() {
        match send_mail("fineyoc13email.net","empty subject","Since this is a simple and common operation, it would be convenient if it could be elided. Alas, because and_then is not sufficiently flexible, it cannot. However, we can instead use ?.") {
          Ok(()) => panic!(""),
          Err(e) => ()
        }

    }
}
