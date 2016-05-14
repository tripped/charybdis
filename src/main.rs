extern crate iron;

use iron::prelude::*;
use iron::status;

fn main() {
    fn hello_world(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "Piss off!")))
    }

    println!("Starting HTTP server on port 1337...");
    Iron::new(hello_world).http("localhost:1337").unwrap();
}
