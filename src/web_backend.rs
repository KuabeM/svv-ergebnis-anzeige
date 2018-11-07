use iron::prelude::*;
use iron::{status, Handler, AroundMiddleware};

use super::errors::*;
use std::io::Read;
use bytes::BufMut;

#[derive(Debug)]
struct ResultDisplay {
    home: u8,
    away: u8,
    minutes: u8,
}

#[derive(Debug)]
struct ResultDisplayHandler<H: Handler> {
    result_disp: ResultDisplay,
    handler: H,
}

impl ResultDisplay {
    fn new(home: u8, away: u8, minutes: u8) -> Self {
        ResultDisplay {home, away, minutes}
    }

    fn show(&self, req: &Request, res: std::result::Result<&Response, &IronError>) {
        println!("request: {:?}, response: {:?}", req, res);
    }

    fn answer(req: &mut Request) -> IronResult<Response> {
        let mut buf = String::new();
        let request_body = req.body.read_to_string(&mut buf);
         Ok(Response::with((status::Ok, buf )))
    }
}

impl<H: Handler> Handler for ResultDisplayHandler<H> {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let res = self.handler.handle(req);
        self.result_disp.show(req, res.as_ref());
        res
    }
}

impl AroundMiddleware for ResultDisplay {
    fn around(self, handler: Box<Handler>) -> Box<Handler> {
        Box::new(
            ResultDisplayHandler {
                result_disp: self,
                handler
            } 
        ) as Box<Handler>
    }
}

fn hello_world(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Hello I'm a rusty backend")))
}

pub fn start() -> Result<()> {

    let res_disp = ResultDisplay::new(5, 1, 30);
    let disp_handle = Iron::new( res_disp.around(Box::new(ResultDisplay::answer )));

    let _disp_listening = disp_handle.http("localhost:3000");


    Ok(())
}