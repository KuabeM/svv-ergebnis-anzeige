use iron::prelude::*;
use iron::{status};

use iron::method::Method;
use iron::error::HttpError;

use std::io::Read;
use super::db::*;

#[derive(Debug, Deserialize,)]
pub struct Minutes {
    min: u8,
}


impl Minutes {
    pub fn new<T>(min: T) -> Self
    where T: Into<u8> {
        Minutes { min: min.into() }
    }

    // fn answer(&mut self, req: &mut Request) -> IronResult<Response> {
    //     let mut buf = String::new();

    //     println!("request {:?}", &req);
    //     req.body.read_to_string(&mut buf)
    //         .map_err(|e| IronError::new(e, "m"))?;
    //     Ok(Response::with((status::Ok,buf)))
    // }

    pub fn post_response(&mut self, request: &mut Request) -> IronResult<Response> {

        match request.method {
            Method::Post => {
                let mut buf = String::new();
                request.body.read_to_string(&mut buf).map_err(|e| IronError::new(e, "Failed to read to string"))?;

                *self = serde_json::from_str(&buf)
                    .map_err(|_| IronError::new(HttpError::Io(std::io::Error::from_raw_os_error(22)), "failed to serialize"))?;

                let db = MysqlDB::db_instance("scoring".to_string()).unwrap();
                db.update(0, "score".to_string(), self.min.clone())
                    .map_err(|_| IronError::new(HttpError::Io(std::io::Error::from_raw_os_error(22)), "faile to insert to database"))?;
                

                let res = Response::with((status::Ok,buf));
                println!("request: {:?}", &request);
                println!("response: {:?}", &res);
                return Ok(res)
            }
            _ => {
                return Err(IronError::new(HttpError::Method, "mee"))
            }
        };
    }
}
