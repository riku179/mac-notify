#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate mac_notify;
extern crate redis;
extern crate rocket;
extern crate r2d2;
extern crate r2d2_redis;

use std::thread;
use std::sync::mpsc::Receiver;
use r2d2_redis::RedisConnectionManager;
use mac_notify::{mac_cap, handlers, redis_client};
use mac_notify::redis_client::User;

fn consumer(ch: Receiver<mac_cap::MacAddr>) -> () {
    match redis_client::get_con("redis://localhost:6379") {
        Ok(con) => loop {
            let mac_addr = ch.recv().unwrap();
//            eprintln!("{}", mac_addr);
            if let Some(user) = User::get_from_redis(&con, mac_addr) {
                User::push_timestamp(&con, &user).unwrap()
            }
        }
        Err(err) => { eprintln!("Failed to connect to redis!"); panic!(err) }
    }
}

// ref) https://rocket.rs/guide/state/#managed-pool
fn init_redis_pool() -> r2d2::Pool<RedisConnectionManager> {
    let manager = RedisConnectionManager::new("redis://localhost")
        .expect("redis connection pool manager");
    r2d2::Pool::builder().build(manager).expect("redis connection pool")
}

fn main() {
    thread::spawn(move || mac_cap::start_capture(consumer));

    rocket::ignite()
        .manage(init_redis_pool())
        .mount("/", routes![
        handlers::get_users,
        handlers::add_user,
        handlers::remove_user
        ])
        .launch();
}
