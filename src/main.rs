extern crate bytes;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate actix;
extern crate actix_web;
extern crate env_logger;
extern crate futures;

extern crate rpi_led_matrix;

extern crate failure;
#[macro_use]
extern crate log;
extern crate docopt;

use docopt::Docopt;

mod errors;
mod panels;
mod web_backend;

use self::errors::*;

const USAGE: &'static str = "
backend-ea

Usage:
  backend-ea (-h | --help)
  backend-ea (-v |--version)
  backend-ea start --ip=<ip> --port=<port>

Commands:
  start         start the backend on ip:port
  
Options:
  --ip=<ip>         ip address no which the backend will be listening
  --port=<port>     port on which the backend will be listening
  -h --help         Show this screen.
  -v --version      Show version.
";

#[derive(Debug, Deserialize)]
struct Args {
    cmd_start: bool,
    flag_ip: String,
    flag_port: String,
    flag_help: bool,
    flag_version: bool,
}

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const NAME: &'static str = env!("CARGO_PKG_NAME");

fn main() -> Result<()> {
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    // check arguments
    if args.flag_version {
        println!("{} {}", NAME, VERSION);
        Ok(())
    } else if args.flag_help {
        println!("{}", USAGE);
        Ok(())
    } else if args.cmd_start{
        
        web_backend::start(args.flag_ip, args.flag_port)?;

        Ok(())
    } else {
        Err(BackendError::ArgumentError
            .context("No idea what you were thinking..")
            .into())
    }

}
