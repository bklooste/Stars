extern crate redis;
extern crate rand;
//
// use redis::Commands;
// use redis::Connection;
// use redis::RedisResult;
//
//
// fn add_score(conn: &Connection, username: &str, score: u32) -> RedisResult<()> {
//     conn.zadd("leaderboard", username, score);
//      Ok(())
// }
//
// type Leaderboard = Vec<(String, u32)>;
//
// fn show_leaderboard(conn: &Connection, n: isize) {
//     let result: RedisResult<Leaderboard> = conn.zrevrange_withscores("leaderboard", 0, n - 1);
//     match result {
//         Ok(board) => {
//             println!("----==== Top {} players ====----", n);
//             for (i, (username, score)) in board.into_iter().enumerate() {
//                 println!("{:<5} {:^20} {:>4}", i + 1, username, score);
//             }
//         },
//         Err(_) => println!("Failed to fetch leaderboard."),
//     }
// }
//
//
// pub fn do_something(conn: &redis::Connection) -> redis::RedisResult<()> {
//
//     let players = vec!["raynor", "kerrigan", "mengsk", "zasz", "tassadar"];
//     for player in players.iter()
//     {
//         let score = rand::random::<u32>() % 1000;
//         add_score(&conn, *player, score).ok().expect("Nuclear launch detected");
//     }
//     show_leaderboard(&conn, 3);
//      Ok(())
// }
//
//
//
// // pub fn getdb() -> redis::RedisResult<()> {
// //     let client = try!(redis::Client::open("redis://127.0.0.1/stars"));
// //
// //     let con = getdb();
// //     //let conn = try!(client.get_connection());
// //
// //     /* do something here */
// //      //
// //     //  let conn = client.get_connection().unwrap();
// //           let _: () = conn.set("answer", 42).unwrap();
// //           let answer: i32 = conn.get("answer").unwrap();
// //          println!("Answer: {}", answer);
// //
// //
// //     Ok(())
// // }
//
// // fn do_something(con: &redis::Connection) -> redis::RedisResult<()> {
// //     let _ : () = try!(con.set("my_key", 42));
// //     Ok(())
// // }
//  */
// //      //
// //     //  let conn = client.get_connection().unwrap();
// //           let _: () = conn.set("answer", 42).unwrap();
// //           let answer: i32 = conn.get("answer").unwrap();
// //          println!("Answer: {}", answer);
// //
// //
// //     Ok(())
// //
// // fn getdb() {
// //     let client = Client::open("redis://127.0.0.1/").unwrap();
// //     let conn = client.get_connection().unwrap();
// //     let _: () = conn.set("answer", 42).unwrap();
// //     let answer: i32 = conn.get("answer").unwrap();
// //     println!("Answer: {}", answer);
// // }

use redis::{Client, Commands, Connection, RedisResult};
use std::collections::HashSet;

//these should be imported
pub type UserId = u32;
pub type RaceId = u32;

#[derive(RustcDecodable, RustcEncodable)]
pub struct  VictoryConditions {
    numWorlds : u32,
}
// VictoryConditions
	// long VCWorlds;
	// long VCTechLevel;
	// long VCTechCount;
	// long VCScore;
	// double VCTimes2nd;
	// long VCResources;
	// long VCCapShips;
	// long VCHighScoreAt;
	// long VCCount;
	// long VCStart;

// mNumberOfPlayers:u32 = 0;
// mWHMin:u32 = 0;
// mWHMax:u32 = 0;
// mWHMinDistance:u32 = 0;

#[derive(RustcDecodable, RustcEncodable)]
pub struct  GameParams {
    numWorlds : u32,
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct  Creation {
    x : u32,
}


pub fn createGame ( gameName :String , Description :String  ,  Notes :String  )
{

}



pub fn createGamewCreation ( gameName : String  ,   gameParams : GameParams , creation :Creation )
{

}


pub fn joinNewGame ( playerid: UserId , myRace: RaceId  )
{

}



pub fn joinExistingGame ( playerid: UserId , myRace: RaceId  )
{

}


fn add_friend(conn: &Connection, my_id: UserId, their_id: UserId) -> RedisResult<()> {
    let my_key = format!("friends:{}", my_id);
    let their_key = format!("friends:{}", their_id);
    try!(conn.sadd(my_key, their_id));
    try!(conn.sadd(their_key, my_id));
    Ok(())
}

fn friends_in_common(conn: &Connection, my_id: UserId, their_id: UserId) -> RedisResult<HashSet<UserId>> {
    let my_key = format!("friends:{}", my_id);
    let their_key = format!("friends:{}", their_id);
    Ok(try!(conn.sinter((my_key, their_key))))
}

fn add_score(conn: &Connection, username: &str, score: u32) -> RedisResult<()> {
    conn.zadd("leaderboard", username, score)
}

type Leaderboard = Vec<(String, u32)>;

fn show_leaderboard(conn: &Connection, n: isize) {
    let result: RedisResult<Leaderboard> = conn.zrevrange_withscores("leaderboard", 0, n - 1);
    match result {
        Ok(board) => {
            println!("----==== Top {} players ====----", n);
            for (i, (username, score)) in board.into_iter().enumerate() {
                println!("{:<5} {:^20} {:>4}", i + 1, username, score);
            }
        },
        Err(_) => println!("Failed to fetch leaderboard."),
    }
}

pub fn do_score() {
    println!("24 days of Rust - redis (day 18)");
    let client = Client::open("redis://127.0.0.1/").unwrap();
    let conn = client.get_connection().unwrap();
    let _: () = conn.set("answer", 42).unwrap();
    let answer: i32 = conn.get("answer").unwrap();
    println!("Answer: {}", answer);

    for i in 1..10u32 {
        add_friend(&conn, i, i + 2).ok().expect("Friendship failed :(");
    }
    println!("You have {} friends in common.",
             friends_in_common(&conn, 2, 3).map(|s| s.len()).unwrap_or(0));

    let players = vec!["raynor", "kerrigan", "mengsk", "zasz", "tassadar"];
    for player in players.iter() {
        let score = rand::random::<u32>() % 1000;
        add_score(&conn, *player, score).ok().expect("Nuclear launch detected");
    }
    show_leaderboard(&conn, 3);
}
