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
extern crate rpi_led_matrix;
#[macro_use]
extern crate common_failures;
#[macro_use]
extern crate failure;

mod display_serve;
mod errors;
mod web_backend;

use self::errors::*;

fn run() -> Result<()> {
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    web_backend::start()?;

    Ok(())
}

quick_main!(run);
