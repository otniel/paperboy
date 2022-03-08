use std::net::TcpListener;

use sqlx::PgPool;

use paperboy::configuration::get_configuration;
use paperboy::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = get_configuration().expect("Failed to read config.");
    let addr = format!("127.0.0.1:{}", config.application_port);

    let listener = TcpListener::bind(addr)?;

    let db_pool = PgPool::connect(&config.database.connection_string())
        .await
        .expect("Failed to connect to Postgres");

    run(listener, db_pool)?.await
}
