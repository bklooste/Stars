extern crate redis;
extern crate rustc_serialize;


extern crate iron;
extern crate router;
extern crate time;
// extern crate serde;
// extern crate serde_json;
/// now r2d2-redis


mod webfascade;
//mod rdbhelp;
// mod queryshipdesign;
// mod queryrace;

mod race;
mod rdbhelp;

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


// fn getPool() -> r2d2::Pool {
//
// }



fn main() {
    println!("starting server");
    let pool = rdbhelp::createPool();

    // web hosting
    let greeting = Arc::new(Mutex::new(Greeting { msg: "Hello, World".to_string() }));
    let greeting_clone = greeting.clone();
    let mut router = Router::new();
// router
    // .get("/", frontpage)
    // .post("login", loginpage);


    router
    .get("/shipsdesigns/:gameid/:playerid/:user", move |r: &mut Request| webfascade::get_ship_designs(r))
//    .get("/shipsdesigns/:gameid/:playerid/:user", move |r: &mut Request| get_ship_designs(r))
    .get("/races/:gameid/:playerid/:user", move |r: &mut Request| webfascade::get_ship_designs(r))
    .post("/set", move |r: &mut Request| set_greeting(r, &mut greeting_clone.lock().unwrap()));


    let mut chain = Chain::new(router);
    chain.link_before(ResponseTime);
    chain.link_after(ResponseTime);

    Iron::new(chain).http("localhost:3000").unwrap();



//    let con = rdbhelp::getConnection(pool);

    //let cloned_pool = pool.clone();
    // for _ in 0..20i32 {
    //     thread::spawn(move || {
    //         let conn = rdbhelp::getConnection(cloned_pool);
    //         //let conn = pool.get().unwrap();
    //         // use the connection
    //         // it will be returned to the pool when it falls out of scope.
    //     });
    // };
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
    println!("finish server");

}
