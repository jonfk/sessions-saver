
#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate iron;
extern crate router;

extern crate serde;
extern crate serde_json;

use iron::prelude::*;
use iron::status;
use router::Router;

use std::io::Read;
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Serialize, Deserialize, Debug)]
struct Greeting {
    msg: String
}

fn main() {
    let greeting = Arc::new(Mutex::new(Greeting { msg: "Hello, World".to_string() }));
    let greeting_clone = greeting.clone();

    let mut router = Router::new();

    router.get("/", move |r: &mut Request| hello_world(r, &greeting.lock().unwrap()));
    router.post("/set", move |r: &mut Request| set_greeting(r, &mut greeting_clone.lock().unwrap()));

    Iron::new(router).http("localhost:3000").unwrap();
    println!("On 3000");
}

fn hello_world(_: &mut Request, greeting: &Greeting) -> IronResult<Response> {
    let payload: String = serde_json::to_string(&greeting).unwrap();
    Ok(Response::with((status::Ok, payload)))
}

// Receive a message by POST and play it back.
fn set_greeting(request: &mut Request, greeting: &mut Greeting) -> IronResult<Response> {
    let mut payload = String::new();
    request.body.read_to_string(&mut payload).unwrap();
    *greeting = serde_json::from_str(&payload).unwrap();
    let payload = serde_json::to_string(&greeting).unwrap();
    Ok(Response::with((status::Ok, payload)))
}
