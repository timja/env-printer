extern crate iron;
extern crate time;

use iron::prelude::*;
use iron::{typemap, AfterMiddleware, BeforeMiddleware};
use std::env;
use time::precise_time_ns;

struct ResponseTime;

impl typemap::Key for ResponseTime {
    type Value = u64;
}

impl BeforeMiddleware for ResponseTime {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        req.extensions.insert::<ResponseTime>(precise_time_ns());
        Ok(())
    }
}

impl AfterMiddleware for ResponseTime {
    fn after(&self, req: &mut Request, res: Response) -> IronResult<Response> {
        let delta = precise_time_ns() - *req.extensions.get::<ResponseTime>().unwrap();
        println!("Request took: {} ms", (delta as f64) / 1000000.0);
        Ok(res)
    }
}

fn get_handler(req: &mut Request) -> IronResult<Response> {
    use params::{Params, Value};

    let map = req.get_ref::<Params>().unwrap();

    let mut strings = env::vars()
        .map(|(a, b)| format!("{}={}\n", a, b))
        .filter(|i| !i.starts_with("LESS_TERMCAP"))
        .collect::<Vec<_>>();

    strings.sort();

    match map.find(&["respond_in"]) {
        Some(&Value::String(ref respond_in)) => {
            let response_int = respond_in.parse::<u64>().unwrap();
            println!("Sleeping for {}s", response_int / 1000);

            use std::{thread, time};
            let sleepy_time = time::Duration::from_millis(response_int);

            thread::sleep(sleepy_time);

            Ok(Response::with((iron::status::Ok, strings.concat())))
        }
        _ => Ok(Response::with((iron::status::Ok, strings.concat()))),
    }
}

fn main() {
    println!("Starting on port 3000");
    let mut chain = Chain::new(get_handler);
    chain.link_before(ResponseTime);
    chain.link_after(ResponseTime);
    Iron::new(chain).http("0.0.0.0:3000").unwrap();
}
