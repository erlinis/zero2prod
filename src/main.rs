use std::net::TcpListener;
use zero2prod::configuration::{self, get_configuration};
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    //Panic if can;t read the configuration
    let configuration = get_configuration().expect("Failed to read configuration");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;

    run(listener)?.await
}
