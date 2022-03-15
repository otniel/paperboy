use std::net::TcpListener;

use sqlx::PgPool;

use paperboy::configuration::get_configuration;
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

    run(listener, db_pool)?.await
}
