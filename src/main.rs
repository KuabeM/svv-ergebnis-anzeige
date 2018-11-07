extern crate iron;
extern crate bytes;
// extern crate rpi_led_matrix;

extern crate failure;

use iron::prelude::*;
use iron::status;

mod web_backend;
mod errors;

use self::errors::*;


fn main() -> Result<()> {
    
    web_backend::start()?;

    Ok(())

}
