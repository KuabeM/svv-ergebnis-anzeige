// extern crate iron;
extern crate bytes;
// extern crate router;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
// extern crate iron_cors;
// extern crate mysql;
extern crate actix;
extern crate actix_web;
extern crate env_logger;
extern crate futures;
// extern crate rpi_led_matrix;

extern crate failure;

mod errors;
mod web_backend;

use self::errors::*;

fn main() -> Result<()> {
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    web_backend::start()?;

    Ok(())
}
