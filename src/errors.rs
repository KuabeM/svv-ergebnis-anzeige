pub use failure::Error;
pub use failure::Fail;

use std;
pub type Result<X> = std::result::Result<X, Error>;

#[derive(Debug, Fail)]
pub enum BackendError {
    #[fail(display = "Failed to init Iron.")]
    IronError,

}