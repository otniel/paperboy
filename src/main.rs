use std::net::TcpListener;

use sqlx::PgPool;

use paperboy::configuration::get_configuration;
use paperboy::email_client::EmailClient;
use paperboy::startup::run;
use paperboy::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let config = get_configuration().expect("Failed to read config.");
    let addr = format!("{}:{}", config.application.host, config.application.port);

    let listener = TcpListener::bind(addr)?;

    let db_pool = PgPool::connect_lazy_with(config.database.with_db());

    let sender = config
        .email_client
        .sender()
        .expect("Invalid sender email address");

    let timeout = config.email_client.timeout();
    let email_client = EmailClient::new(
        config.email_client.base_url,
        sender,
        config.email_client.authorization_token,
        timeout,
    );

    run(listener, db_pool, email_client)?.await
}
