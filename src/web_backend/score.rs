use actix_web::{AsyncResponder, Error, HttpMessage, HttpRequest, HttpResponse};
use futures::Future;

use super::db::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Score {
    score_home: u8,
    score_away: u8,
}

impl Score {
    pub fn score(req: &HttpRequest) -> Box<Future<Item = HttpResponse, Error = Error>> {
        req.json()
            .from_err() // convert all errors into `Error`
            .and_then(|val: Score| {
                println!("model: {:?}", val);
                let db = MysqlDB::db_instance("scoring".to_string()).unwrap();
                db.update(1, "score".to_string(), val.score_home.clone())?;
                db.update(2, "score".to_string(), val.score_away.clone())?;

                Ok(HttpResponse::Ok().json(val)) // <- send response
            })
            .responder()
    }
}
