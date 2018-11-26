use iron::prelude::*;
use iron::{status, Handler, AroundMiddleware};

use iron::headers::{AccessControlAllowOrigin, ContentType, };
use iron::method::Method;
use iron::error::HttpError;

use super::errors::*;
use std::io::Read;
use bytes::BufMut;
use router::Router;
use std::sync::{Arc,Mutex};

#[derive(Debug, Serialize, Deserialize,)]
struct ResultDisplay {
    home: u8,
    away: u8,
    minutes: u8,
}

// #[derive(Debug)]
// struct ResultDisplayHandler<H: Handler> {
//     result_disp: ResultDisplay,
//     handler: H,
// }

impl ResultDisplay {
    fn new(home: u8, away: u8, minutes: u8) -> Self {
        ResultDisplay {home, away, minutes}
    }

    fn show(&self, req: &Request, res: std::result::Result<&Response, &IronError>) {
        println!("request: {:?}, response: {:?}", req, res);
    }

    fn answer(&mut self, req: &mut Request) -> IronResult<Response> {
        let mut buf = String::new();

        println!("{:?}",req.method);
        req.body.read_to_string(&mut buf)
            .map_err(|e| IronError::new(e, "m"))?;
            // .map_err(|e| BackendError::IronError)?;
        println!("req: {:?}", &req);
        let mut res = Response::with((status::Ok,buf));
        res.headers.set(AccessControlAllowOrigin::Any);
        Ok(res)
    }

    fn post_response(&mut self, request: &mut Request) -> IronResult<Response> {
        // let mut payload = serde_json::to_string(&self).map_err(|e| IronError::new(e, "m"))?;

        match request.method {
            Method::Options => {
                let mut res = Response::with(status::Ok);
                res.headers.set(AccessControlAllowOrigin::Any);
                res.headers.set_raw("Access-Control-Allow-Methods",
                    vec![b"POST, GET, OPTIONS".to_vec()]
                );
                res.headers.set_raw("Access-Control-Allow-Headers",
                    vec![b"Content-Type".to_vec()]
                );
                println!("request: {:?}", &self);
                println!("response: {:?}", &res);
                return Ok(res)
            }
            Method::Post => {
                let mut buf = String::new();
                request.body.read_to_string(&mut buf).map_err(|e| IronError::new(e, "m"))?;
                *self = serde_json::from_str(&buf).map_err(|e| IronError::new(e, "mee"))?;
                let mut res = Response::with((status::Ok,buf));
                println!("request: {:?}", &request);
                println!("response: {:?}", &res);
                return Ok(res)
            }
            _ => {
                return Err(IronError::new(HttpError::Method, "mee"))
            }
        };

        // let mut buf = String::new();
        // request.body.read_to_string(&mut buf).map_err(|e| IronError::new(e, "m"))?;
        // *self = serde_json::from_str(&buf).map_err(|e| IronError::new(e, "mee"))?;
        // let mut res = Response::with((status::Ok,buf));
        // res.headers.set(AccessControlAllowOrigin::Any);//Value("localhost".to_owned()));
        // res.headers.set_raw("Access-Control-Allow-Headers",
        //     vec![b"X-PINGOTHER, Content-Type".to_vec()]
        // );
        // res.headers.set_raw("Access-Control-Allow-Methods",
        //     vec![b"POST, GET, OPTIONS".to_vec()]
        // );
        // res.headers.s

        // println!("request: {:?}", &self);
        // println!("response: {:?}", &res);
        // Ok(res)
    }
}

// impl<H: Handler> Handler for ResultDisplayHandler<H> {
//     fn handle(&self, req: &mut Request) -> IronResult<Response> {
//         let res = self.handler.handle(req);
//         self.result_disp.show(req, res.as_ref());
//         res
//     }
// }

// impl AroundMiddleware for ResultDisplay {
//     fn around(self, handler: Box<Handler>) -> Box<Handler> {
//         Box::new(
//             ResultDisplayHandler {
//                 result_disp: self,
//                 handler
//             } 
//         ) as Box<Handler>
//     }
// }

// fn hello_world(_: &mut Request) -> IronResult<Response> {
//     Ok(Response::with((status::Ok, "Hello I'm a rusty backend")))
// }


pub fn start() -> Result<()> {

    let mut router = Router::new();
    let res_disp = Arc::new(Mutex::new(ResultDisplay::new(5, 1, 30)));
    let res_disp_clone = res_disp.clone();

    router.get("/", move |r: &mut Request| {
        res_disp.lock()
            .unwrap()
            .answer(r)
    }, "get");

    router.post("/competitors", move |r: &mut Request| {
        res_disp_clone.lock()
            .unwrap()
            .post_response(r)
    }, "posting");

    Iron::new(router).http("192.168.0.50:3000")?;
    // let disp_handle = Iron::new( res_disp.around(Box::new(ResultDisplay::answer )));

    // let _disp_listening = disp_handle.http("localhost:3000")?;


    Ok(())
}