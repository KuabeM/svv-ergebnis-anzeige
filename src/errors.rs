pub use failure::Error;
pub use failure::Fail;

use std;
pub type Result<X> = std::result::Result<X, Error>;

#[derive(Debug, Fail)]
pub enum BackendError {
    #[fail(display = "Failed to connect to database")]
    DatabaseError,

    #[fail(display = "Failed to parse arguments")]
    ArgumentError,
}

// impl error::ResponseError for BackendError {
//     fn error_response(&self) -> HttpResponse {
//         match *self {
//             BackendError::DatabaseError => {
//                 HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR)
//             }
//         }
//     }
// }

#[derive(Debug, Fail)]
pub enum PanelError {
    #[fail(display = "Failed to init panel")]
    InitError,
}
