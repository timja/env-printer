extern crate iron;
extern crate time;

use iron::prelude::*;
use iron::{BeforeMiddleware, AfterMiddleware, typemap};
use time::precise_time_ns;
use std::env;

struct ResponseTime;

impl typemap::Key for ResponseTime { type Value = u64; }

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

fn hello_world(_: &mut Request) -> IronResult<Response> {
    // for (key, value) in env::vars() {
    //     println!("{}: {}", key, value);
    // }
    let mut strings = env::vars()
    .map(|(a, b)| format!("{}={}\n", a, b))
                 .filter(|i|!i.starts_with("LESS_TERMCAP"))
                 .collect::<Vec<_>>();
                 
    strings.sort();

    Ok(Response::with((iron::status::Ok, strings.concat())))
}

fn main() {
    println!("Starting on port 3000");
    let mut chain = Chain::new(hello_world);
    chain.link_before(ResponseTime);
    chain.link_after(ResponseTime);
    Iron::new(chain).http("0.0.0.0:3000").unwrap();
}
