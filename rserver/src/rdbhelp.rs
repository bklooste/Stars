extern crate redis;
use redis::Commands;
use redis::Connection;

pub fn getconnection() -> redis::Connection {
    let client = redis::Client::open("redis://127.0.0.1/1").unwrap(); // 1 is db #
    //let conn = try!(client.get_connection());
    return  client.get_connection().unwrap();

    //con;
    //return conn.unwrap();
    // /* do something here */
    //  //
    // //  let conn = client.get_connection().unwrap();
    //       let _: () = conn.set("answer", 42).unwrap();
    //       let answer: i32 = conn.get("answer").unwrap();
    //      println!("Answer: {}", answer);
    //
    //
    //Ok(())
}
