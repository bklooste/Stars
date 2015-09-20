extern crate redis;
extern crate r2d2;
extern crate r2d2_redis;

use redis::Commands;
use redis::Connection;
use self::r2d2::{Pool , PooledConnection};
use self::r2d2_redis::RedisConnectionManager;

pub type RedisPooledConnection = r2d2::PooledConnection<RedisConnectionManager>;
pub type RedisPool = r2d2::Pool<RedisConnectionManager>;

//static pool :  RedisPool = init();


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


pub fn createPool() -> RedisPool {
// pool
    let config = r2d2::Config::builder()
        .error_handler(Box::new(r2d2::LoggingErrorHandler))
        .build();
    let manager = r2d2_redis::RedisConnectionManager::new("redis://127.0.0.1/").unwrap();
    return r2d2::Pool::new(config, manager).unwrap();
}

pub fn getConnection( pool : RedisPool) -> RedisPooledConnection
{
    return pool.get().unwrap();
}
