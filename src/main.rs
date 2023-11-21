use newsletter_service::configuration::get_configuration;
use newsletter_service::startup::run;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    run(TcpListener::bind(address).expect("Failed to bind port"))?.await
}
