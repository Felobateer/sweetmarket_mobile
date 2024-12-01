use lettre::{Message, SmtpTransport, Transport};
use lettre::transport::smtp::authentication::Credentials;
use std::env;

pub struct EmailService;

impl EmailService {
    pub fn deliver_email(dest: &str, subject: &str, body: &str) {
        // Set up the SMTP client using environment variables for security
        let email = env::var("EMAIL").expect("EMAIL must be set");
        let email_pwd = env::var("EMAIL_PWD").expect("EMAIL_PWD must be set");
        let smtp_server = env::var("SMTP_SERVER").expect("SMTP_SERVER must be set");
        let smtp_port = env::var("SMTP_PORT").unwrap_or("587".to_string()); // Default to 587 if not provided

        // Create SMTP credentials
        let creds = Credentials::new(email.clone(), email_pwd);

        // Create the email message
        let email = Message::builder()
            .from(email.clone().parse().unwrap())
            .to(dest.parse().unwrap())
            .subject(subject)
            .body(body.to_string())
            .unwrap();

        // Create the transport to send the email
        let mailer = SmtpTransport::relay(&smtp_server)
            .unwrap()
            .credentials(creds)
            .port(smtp_port.parse().unwrap())
            .build();

        // Send the email
        match mailer.send(&email) {
            Ok(_) => println!("Email sent to: {}", dest),
            Err(e) => eprintln!("Error sending email: {}", e),
        }
    }
}
