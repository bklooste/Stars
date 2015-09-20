extern crate redis;

use redis::Commands;
use redis::Connection;


//let con = rdbhelp::getconnection();



pub fn do_something(conn: &redis::Connection) -> redis::RedisResult<()> {
    //let _ : () = try!(con.set("my_key", 42));

     let _: () = conn.set("answer", 42).unwrap();
     let answer: i32 = conn.get("answer").unwrap();
     println!("Answer: {}", answer);
    Ok(())
}



// pub fn getdb() -> redis::RedisResult<()> {
//     let client = try!(redis::Client::open("redis://127.0.0.1/stars"));
//
//     let con = getdb();
//     //let conn = try!(client.get_connection());
//.get("/shipsdesigns/:gameid/:playerid/:user", move |r: &mut Request| get_ship_designs(r))
//     /* do something here */
//      //
//     //  let conn = client.get_connection().unwrap();
//           let _: () = conn.set("answer", 42).unwrap();
//           let answer: i32 = conn.get("answer").unwrap();
//          println!("Answer: {}", answer);
//
//
//     Ok(())
// }

// fn do_something(con: &redis::Connection) -> redis::RedisResult<()> {
//     let _ : () = try!(con.set("my_key", 42));
//     Ok(())
// }

//
// fn getdb() {
//     let client = Client::open("redis://127.0.0.1/").unwrap();
//     let conn = client.get_connection().unwrap();
//     let _: () = conn.set("answer", 42).unwrap();
//     let answer: i32 = conn.get("answer").unwrap();
//     println!("Answer: {}", answer);
// }
