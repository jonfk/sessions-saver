#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate crypto;

extern crate dotenv;
extern crate iron;
extern crate router;

extern crate serde;
extern crate serde_json;

extern crate postgres;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate iron_postgres_middleware as pg_middleware;
use pg_middleware::{PostgresMiddleware, PostgresReqExt};
use r2d2::{Pool, PooledConnection};
use r2d2_postgres::{PostgresConnectionManager};

mod store;
use store::users;

mod auth;
extern crate jsonwebtoken as jwt;
extern crate rustc_serialize;

mod errors;

use dotenv::dotenv;
use std::env;

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

const DATABASE_URL: &'static str = "DATABASE_URL";
const SERVER_ADDR: &'static str = "SESSION_SAVER_ADDR";

//pub type PostgresPool = Pool<PostgresConnectionManager>;
pub type PostgresPooledConnection = PooledConnection<PostgresConnectionManager>;

fn main() {
    dotenv().ok();

    let greeting = Arc::new(Mutex::new(Greeting { msg: "Hello, World".to_string() }));
    let greeting_clone = greeting.clone();

    let mut router = Router::new();

    router.get("/", move |r: &mut Request| hello_world(r, &greeting.lock().unwrap()));
    router.post("/signup", create_user);
    router.post("/login", login);

    let mut c = Chain::new(router);

    match env::var(DATABASE_URL) {
        Ok(val) => {
            let pg_middleware = PostgresMiddleware::new(&val).unwrap();
            c.link_before(pg_middleware);
        },
        Err(_) => {
            println!("DATABASE_URL env variable is not set");
            return;
        },
    }

    match env::var(SERVER_ADDR) {
        Ok(val) => {
            let addr: &str = &val;
            Iron::new(c).http(addr).unwrap();
            println!("On {}", addr);
        },
        Err(_) => {
            println!("{} env variable is not set", SERVER_ADDR);
            return;
        },
    }
}

fn hello_world(_: &mut Request, greeting: &Greeting) -> IronResult<Response> {
    let payload: String = serde_json::to_string(&greeting).unwrap();
    Ok(Response::with((status::Ok, payload)))
}


fn create_user(request: &mut Request) -> IronResult<Response> {
    let mut payload = String::new();
    request.body.read_to_string(&mut payload).unwrap();
    let user: users::User = serde_json::from_str(&payload).unwrap();

    let conn = request.db_conn();
    let a = users::create_user(&conn, user.email, user.password).unwrap();

    Ok(Response::with((status::Ok, "Account created")))
}

fn login(request: &mut Request) -> IronResult<Response> {
    let mut payload = String::new();
    request.body.read_to_string(&mut payload).unwrap();
    let user: users::User = serde_json::from_str(&payload).unwrap();

    let conn = request.db_conn();
    match auth::login(&conn, user.email, user.password) {
        Ok(login) => {
            let result: String = serde_json::to_string(&login).unwrap();
            Ok(Response::with((status::Ok, result)))
        },
        Err(err) => {
            let result: String = serde_json::to_string(&auth::Login{success: false, token:"".to_string()}).unwrap();
            // TODO log err
            Ok(Response::with((status::Ok, result)))
        }

    }

}
