use lettre::{transport::smtp::{authentication::{Credentials, Mechanism}, PoolConfig}, Message, SmtpTransport, Transport};
use std::env;

#[allow(dead_code)]
pub fn send_email(
    from: String,
    to: String,
    subject: String,
    body: String
) {
    let username = env::var("SMTP_USERNAME").unwrap();
    let password = env::var("SMTP_PASSWORD").unwrap();
    let port = env::var("SMTP_PORT").unwrap();
    let host = env::var("SMTP_HOST").unwrap();
    let email = Message::builder()
        .from(from.parse().unwrap())
        .to(to.parse().unwrap())
        .subject(subject)
        .body(body).unwrap();
    let sender = SmtpTransport::starttls_relay(&host).unwrap()
        .credentials(Credentials::new(
            username.to_owned(),
            password.to_owned()
        ))
        .authentication(vec![Mechanism::Plain])
        .pool_config(PoolConfig::new().max_size(20))
        .port(port.parse().unwrap())
        .build();
    
    let result = sender.send(&email);
    println!("is sent ok ? {}", result.is_ok());
}

pub fn send_email_v2(
    from: &str,
    to: &str,
    subject: &str,
    body: &str
) {
    let username = env::var("SMTP_USERNAME").unwrap();
    let password = env::var("SMTP_PASSWORD").unwrap();
    let port = env::var("SMTP_PORT").unwrap();
    let host = env::var("SMTP_HOST").unwrap();
    let email = Message::builder()
        .from(from.parse().unwrap())
        .to(to.parse().unwrap())
        .subject(subject)
        .body(body.to_string()).unwrap();
    let sender = SmtpTransport::starttls_relay(&host).unwrap()
        .credentials(Credentials::new(
            username.to_owned(),
            password.to_owned()
        ))
        .authentication(vec![Mechanism::Plain])
        .pool_config(PoolConfig::new().max_size(20))
        .port(port.parse().unwrap())
        .build();
    
    let result = sender.send(&email);
    println!("is sent ok ? {}", result.is_ok());
}