extern crate iron;
extern crate bytes;
extern crate router;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate iron_cors;
extern crate mysql;

// extern crate rpi_led_matrix;

extern crate failure;


mod web_backend;
mod errors;

use self::errors::*;


fn main() -> Result<()> {
    
    web_backend::start()?;

    Ok(())

}
