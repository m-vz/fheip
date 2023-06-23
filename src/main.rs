use std::error::Error;

mod exploration;
mod server;

fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();

    server::start("127.0.0.1:34347")
}
