use super::errors::*;

use actix_web::{http, http::header, middleware, middleware::cors::Cors, server, App};

mod db;
mod minutes;
mod score;
mod teams;
use self::db::*;
use self::minutes::Minutes;
use self::score::Score;
use self::teams::Teams;

pub fn start() -> Result<()> {
    let sys = actix::System::new("results");

    let db = MysqlDB::create_db("scoring".to_string())?;
    db.insert(0, "time".to_string(), 0)?;
    db.insert(1, "SVV".to_string(), 0)?;
    db.insert(2, "Gast".to_string(), 0)?;

    server::new(|| {
        App::new()
            .middleware(middleware::Logger::default())
            .configure(|app| {
                Cors::for_app(app)
                    .allowed_origin("http://localhost:8080")
                    .allowed_methods(vec!["POST"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .max_age(3600)
                    .resource("/score", |r| r.method(http::Method::POST).f(Score::score))
                    .resource("/minutes", |r| {
                        r.method(http::Method::POST).f(Minutes::minutes)
                    })
                    .resource("/teams", |r| r.method(http::Method::POST).f(Teams::teams))
                    .register()
            })
    })
    .bind("192.168.0.41:3000")
    .unwrap()
    .start();

    println!("Started http server on 192.168.0.41:3000");
    let _ = sys.run();

    Ok(())
}
