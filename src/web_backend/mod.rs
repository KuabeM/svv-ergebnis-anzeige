use super::errors::*;
use std::env::var;

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

    let ip_var = var("IP_ADDR").unwrap_or("localhost".to_string());
    let port_var = var("PORT").unwrap_or("3000".to_string());
    let url = format!("{}:{}", ip_var, port_var);
    let allow_orig = format!("http://{}:8080", ip_var);

    server::new(move || {
        App::new()
            .middleware(middleware::Logger::default())
            .configure(|app| {
                Cors::for_app(app)
                    .allowed_origin(allow_orig.as_str())
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
    .bind(&url)
    .unwrap()
    .start();

    println!("Started http server on {}:{}", ip_var, port_var);
    let _ = sys.run();

    Ok(())
}
