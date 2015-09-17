extern crate redis;
extern crate rustc_serialize;


extern crate iron;
extern crate router;
extern crate time;
// extern crate serde;
// extern crate serde_json;
/// now r2d2-redis
extern crate r2d2;
extern crate r2d2_redis;


mod rdbhelp;
mod queryshipdesign;
mod gamemanage;

use iron::prelude::*;
use iron::{BeforeMiddleware, AfterMiddleware, typemap,status};
use time::precise_time_ns;
use router::Router;

use std::io::Read;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

use rustc_serialize::json;

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


//#[derive(Serialize, Deserialize, Debug)]
#[derive(RustcDecodable, RustcEncodable)]
struct Greeting {
    msg: String
}

//let payload = json::encode(&greeting).unwrap();NewReports = 1

fn get_ship_designs(req: &mut Request) -> IronResult<Response> {

let con = rdbhelp::getconnection();

queryshipdesign::do_something(&con);
    let ref gameid  = req.extensions.get::<Router>().unwrap().find("gameid").unwrap_or("/");
//        let ref gameid  = req.extensions.get::<Router>().unwrap().find("gameid").unwrap_or("/");

    gamemanage::do_score();
    println!("gameid: {}", gameid);
//        println!("userid {}",  userid);
    // validate gameid

//request.body
      let payload = json::encode(gameid).unwrap();
      Ok(Response::with((status::Ok, payload)))
  }

// fn getPool() -> r2d2::Pool {
//
// }


fn main() {
    // pool
    let config = r2d2::Config::builder()
        .error_handler(Box::new(r2d2::LoggingErrorHandler))
        .build();
    let manager = r2d2_redis::RedisConnectionManager::new("localhost:1234").unwrap();
    let pool = r2d2::Pool::new(config, manager).unwrap();


    // web hosting
    let greeting = Arc::new(Mutex::new(Greeting { msg: "Hello, World".to_string() }));
    let greeting_clone = greeting.clone();
    let mut router = Router::new();
// router
    // .get("/", frontpage)
    // .post("login", loginpage);

    router
    .get("/shipsdesigns/:gameid/:playerid/:user", move |r: &mut Request| get_ship_designs(r))
    .post("/set", move |r: &mut Request| set_greeting(r, &mut greeting_clone.lock().unwrap()));


    let mut chain = Chain::new(router);
    chain.link_before(ResponseTime);
    chain.link_after(ResponseTime);
    Iron::new(chain).http("localhost:3000").unwrap();



    for _ in 0..20i32 {
        let pool = pool.clone();
        thread::spawn(move || {
            let conn = pool.get().unwrap();
            // use the connection
            // it will be returned to the pool when it falls out of scope.
        });
    };
    //
    // finish



    //  let mut router = Router::new();


    //  "/shipsdesigns/:gameid/:playerid/:user"
    // map
    // planets
    // characters
    // componentsw
    // fleets
    // tech
    // races






//http://ironframework.io/doc/router/struct.Router.html
    fn hello_world(req: &mut Request, greeting: &Greeting) -> IronResult<Response> {

        let ref gameid  = req.extensions.get::<Router>().unwrap().find("gameid").unwrap_or("/");
//        let ref gameid  = req.extensions.get::<Router>().unwrap().find("gameid").unwrap_or("/");

        println!("gameid: {}", gameid);
//        println!("userid {}",  userid);
        // validate gameid


//          let ref userid  = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("/");
// print it
          //request.body
          let payload = json::encode(&greeting).unwrap();
          Ok(Response::with((status::Ok, payload)))
      }

    // Receive a message by POST and play it back.
    fn set_greeting(request: &mut Request , greeting: &mut Greeting) -> IronResult<Response> {
        let mut payload = String::new();
        request.body.read_to_string(&mut payload).unwrap();
        //let request: Greeting = json::decode(&payload).unwrap();
        //let greeting = Greeting { msg: request.msg };
        //let payload = json::to_string(&greeting).unwrap();
        *greeting = json::decode(&payload).unwrap();
        Ok(Response::with(status::Ok))
    }


    //Iron::new(router).http("localhost:3000").unwrap();
}
