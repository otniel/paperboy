use paperboy::configuration::get_configuration;
use paperboy::startup::Application;
use paperboy::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("paperboy".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let config = get_configuration().expect("Failed to read config.");
    let app = Application::build(config).await?;
    app.run_until_stopped().await?;
    Ok(())
}
