use actix_web::{AsyncResponder, Error, HttpMessage, HttpRequest, HttpResponse};
use futures::Future;

use super::db::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct Minutes {
    min: u8,
}

impl Minutes {
    pub fn minutes(req: &HttpRequest) -> Box<Future<Item = HttpResponse, Error = Error>> {
        req.json()
            .from_err() // convert all errors into `Error`
            .and_then(|val: Minutes| {
                println!("model: {:?}", val);
                let db = MysqlDB::db_instance("scoring".to_string()).unwrap();
                db.update(0, "score".to_string(), val.min.clone())?;

                Ok(HttpResponse::Ok().json(val)) // <- send response
            })
            .responder()
    }
}
