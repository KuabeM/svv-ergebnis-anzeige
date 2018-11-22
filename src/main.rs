extern crate iron;
extern crate bytes;
// extern crate rpi_led_matrix;

extern crate failure;


mod web_backend;
mod errors;

use self::errors::*;


fn main() -> Result<()> {
    
    web_backend::start()?;

    Ok(())

}
