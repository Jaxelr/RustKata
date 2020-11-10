use log::{error, warn, info, debug, trace};

fn main() {

    //In order for this to run, you need to enable the RUST_LOG env var
    //env RUST_LOG=trace cargo run
    pretty_env_logger::init();

    trace!("starting main method");
    debug!("Debugging message");

    info!("This is an information message");

    warn!("Warning, something is about to go wrong!");
    error!("This is an error man");
}
