extern crate lettre;
extern crate lettre_email;
use lettre::smtp::authentication::IntoCredentials;
use lettre::{SmtpClient, Transport};
use lettre_email::EmailBuilder;

pub fn send_mail() {
    let smtp_address = "smtp.gmail.com";
    let username = "anomaly.detector2020@gmail.com";
    let password = "anomalydetector2020*";
    let email = EmailBuilder::new()
        .to("hajnal.andor1993@gmail.com")
        .from(username)
        .subject("Hi Ferenc")
        .text("“In a study, scientists report that drinking beer can be good for the liver. I’m sorry, did I say ‘scientists’? I meant Irish people.")
        .build()
        .unwrap()
        .into();
    let credentials = (username, password).into_credentials();
    let mut client = SmtpClient::new_simple(smtp_address)
        .unwrap()
        .credentials(credentials)
        .transport();
    let result = client.send(email);
      if result.is_ok() {
         println!("Email sent");
      } else {
        println!("Could not send email: {:?}", result);
      }
}


// // use lettre::{EmailTransport, SmtpTransport};
// extern crate lettre_email;
// use lettre::{Message, Envelope, Transport, SendmailTransport,SmtpClient,SendableEmail, EmailAddress};
// use lettre_email::Email;
// use std::path::Path;

// pub fn send_mail() {
//   // let email = EmailBuilder::new()
//   //   // Addresses can be specified by the tuple (email, alias)
//   //   .to(("user@example.org", "Firstname Lastname"))
//   //   // ... or by an address only
//   //   .from("user@example.com")
//   //   .subject("Hi, Hello world")
//   //   .text("Hello world.")
//   //   .build()
//   //   .unwrap();
//   let email = SendableEmail::new(
//     Envelope::new(
//         Some(EmailAddress::new("user@localhost".to_string()).unwrap()),
//         vec![EmailAddress::new("root@localhost".to_string()).unwrap()],
//     ).unwrap(),
//     "id12321".to_string(),
//     "Message123212321232".to_string().into_bytes(),
// );
//   // Open a local connection on port 25
//   let mut mailer = SmtpClient::new_unencrypted_localhost().unwrap().transport();
//   // Send the email
//   let result = mailer.send(email);
//   if result.is_ok() {
//      println!("Email sent");
//   } else {
//     println!("Could not send email: {:?}", result);
//   }
// }





// use lettre::{EmailTransport, SmtpTransport};
// use lettre_email::EmailBuilder;
// use std::path::Path;


// pub fn send_mail() {
//   let email = EmailBuilder::new()
//   // Addresses can be specified by the tuple (email, alias)
//   .to(("user@example.org", "Firstname Lastname"))
//   // ... or by an address only
//   .from("user@example.com")
//   .subject("Hi, Hello world")
//   .text("Hello world.")
//   .build()
//   .unwrap();
  
//   // Open a local connection on port 25
//   let mut mailer = SmtpTransport::builder_unencrypted_localhost().unwrap()
//   .build();
//   // Send the email
//   let result = mailer.send(&email);
  
//   if result.is_ok() {
//   println!("Email sent");
//   } else {
//   println!("Could not send email: {:?}", result);
//   }
  
//   assert!(result.is_ok());
// }
