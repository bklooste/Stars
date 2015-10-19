extern crate redis;
extern crate rustc_serialize;


extern crate iron;
extern crate persistent;
extern crate router;
extern crate time;
extern crate uuid;
// extern crate serde;
// extern crate serde_json;
/// now r2d2-redis

mod cqrs {
    pub mod aggregate;
}

mod utils {
    pub mod logging;
    pub mod error;
}

mod bl {
    pub mod race;
}

mod webfascade;
mod rdbhelp;

use iron::prelude::*;
use iron::{BeforeMiddleware, AfterMiddleware, typemap,status};
use router::Router;
use time::precise_time_ns;
use std::env;
//use std::f64;
use webfascade::AppDb;

use cqrs::aggregate::Circle;



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

fn environment(_: &mut Request) -> IronResult<Response> {
    let powered_by:String = match env::var("POWERED_BY") {
        Ok(val) => val ,
        Err(_) => "Iron".to_string()
    };
    let message = format!("Powered by: {}, pretty cool aye", powered_by);
    Ok(Response::with((status::Ok, message)))
}


fn main() {

    //std::f64::consts::
    let c = Circle::new( 0.0, 0.0, 2.0 ); //often use new
    //let c = aggregate::Circle { x: 0.0, y: 0.0, radius: 2.0 }; //often use new
    println!("{}", c.area());



    println!("starting server");
    let pool = rdbhelp::createPool();

    let mut router = Router::new();

    router.get("/", environment);
    router.get("/shipsdesigns/:gameid/:playerid/:user", move |r: &mut Request| webfascade::get_ship_designs(r));
    router.get("/races/:gameid/:playerid/:user", move |r: &mut Request| webfascade::get_race_designs(r )); //   &pool.get().unwrap()))  //all
    router.post("/races/:gameid/:playerid/:user", move |r: &mut Request| webfascade::set_race_designs(r ));
//    router.post("/set", move |r: &mut Request| set_greeting(r, &mut greeting_clone.lock().unwrap()));


    let mut chain = Chain::new(router);
    chain.link_before(ResponseTime);
    chain.link(persistent::Read::<AppDb>::both(pool));
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






    // fn hello_world(req: &mut Request, greeting: &Greeting) -> IronResult<Response> {
    //
    //     let ref gameid  = req.extensions.get::<Router>().unwrap().find("gameid").unwrap_or("/");
    //     println!("gameid: {}", gameid);
    //       let payload = json::encode(&greeting).unwrap();
    //       Ok(Response::with((status::Ok, payload)))
    //   }

    // // Receive a message by POST and play it back.
    // fn set_greeting(request: &mut Request , greeting: &mut Greeting) -> IronResult<Response> {
    //     let mut payload = String::new();
    //     request.body.read_to_string(&mut payload).unwrap();
    //     //let request: Greeting = json::decode(&payload).unwrap();
    //     //let greeting = Greeting { msg: request.msg };
    //     //let payload = json::to_string(&greeting).unwrap();
    //     *greeting = json::decode(&payload).unwrap();
    //     Ok(Response::with(status::Ok))
    // }


    //Iron::new(router).http("localhost:3000").unwrap();
    println!("finish server");

}
