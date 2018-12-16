use actix_web::{AsyncResponder, Error, HttpMessage, HttpRequest, HttpResponse};
use futures::Future;

use super::db::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Teams {
    name_home: String,
    name_away: String,
}

impl Teams {
    pub fn teams(req: &HttpRequest) -> Box<Future<Item = HttpResponse, Error = Error>> {
        req.json()
            .from_err() // convert all errors into `Error`
            .and_then(|val: Teams| {
                println!("model: {:?}", val);
                let db = MysqlDB::db_instance("scoring".to_string()).unwrap();
                db.update(1, "team".to_string(), val.name_home.clone())?;
                db.update(2, "team".to_string(), val.name_away.clone())?;

                Ok(HttpResponse::Ok().json(val)) // <- send response
            })
            .responder()
    }
}
