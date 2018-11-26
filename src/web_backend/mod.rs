use iron::prelude::*;
// use iron::{status};

// use iron::method::Method;
// use iron::error::HttpError;

use super::errors::*;
// use std::io::Read;
use router::Router;
use std::sync::{Arc,Mutex};

use iron_cors::CorsMiddleware;
use std::collections::HashSet;


mod score;
mod teams;
mod db;
mod minutes;
use self::score::Score;
use self::teams::Teams;
use self::minutes::Minutes;
use self::db::*;



pub fn start() -> Result<()> {

    let mut router = Router::new();
    let post_teams = Arc::new(Mutex::new(Teams::new("SVV".to_string(), "BCA".to_string())));
    let post_score = Arc::new(Mutex::new(Score::new(0, 0)));
    let post_minutes = Arc::new(Mutex::new(Minutes::new(0)));

    let db = MysqlDB::create_db("scoring".to_string())?;
    db.insert(0, "time".to_string(), 0)?;
    db.insert(1, "SVV".to_string(), 0)?;
    db.insert(2, "Gast".to_string(), 0)?;
    
    // router.get("/", move |r: &mut Request| {
    //     res_disp_score.clone().lock()
    //         .unwrap()
    //         .answer(r)
    // }, "get");

    router.post("/teams", move |r: &mut Request| {
        post_teams.lock()
            .unwrap()
            .post_response(r)
    }, "teams");

    router.post("/score", move |r: &mut Request| {
        post_score.lock()
            .unwrap()
            .post_response(r)
    }, "score");

    router.post("/minutes", move |r: &mut Request| {
        post_minutes.lock()
            .unwrap()
            .post_response(r)
    }, "minutes");

    let allowed_hosts = ["http://192.168.0.50:8080", "http://localhost:8080"].iter().map(ToString::to_string).collect::<HashSet<_>>();

    let cors = CorsMiddleware::with_whitelist(allowed_hosts);

    let mut chain = Chain::new(router);
    chain.link_around(cors);

    Iron::new(chain).http("192.168.0.50:4000")?;


    Ok(())
}