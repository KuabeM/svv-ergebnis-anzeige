use iron::prelude::*;
use iron::{status};

use iron::method::Method;
use iron::error::HttpError;

use std::io::Read;
use super::db::*;


#[derive(Debug, Serialize, Deserialize,)]
pub struct Score {
    score_home: u8,
    score_away: u8,
}

// #[derive(Debug)]
// struct ScoreHandler<H: Handler> {
//     result_disp: Score,
//     handler: H,
// }

impl Score {
    pub fn new(score_home: u8, score_away: u8) -> Self {
        Score {score_home, score_away}
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
                db.update(1, "score".to_string(), self.score_home.clone())
                    .map_err(|_| IronError::new(HttpError::Io(std::io::Error::from_raw_os_error(22)), "faile to insert to database"))?;
                db.update(2, "score".to_string(), self.score_away.clone())
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