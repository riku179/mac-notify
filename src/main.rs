#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate mac_notify;
extern crate redis;
extern crate rocket;

use std::thread;
use std::sync::mpsc::Receiver;
use mac_notify::{mac_cap, handlers, redis_client, model};
use mac_notify::redis_client::User;

fn consumer(ch: Receiver<mac_cap::MacAddr>, con: redis::Connection) -> () {
    loop {
        let mac_addr = ch.recv().unwrap();
        if let Some(user) = User::get_from_redis(&con, mac_addr) {
            User::push_timestamp(&con, &user).unwrap()
        }
    }
}

fn main() {
    let redis_con = redis_client::get_con("redis://localhost:6379")
        .expect("Failed to get redis connection");

    thread::spawn(move || mac_cap::start_capture(
        consumer, redis_con)
    );

    let redis_pool = redis_client::new_redis_pool("redis://localhost:6379")
        .expect("redis connection pool");
    let mysql_pool = model::new_mysql_pool("mysql://hogefuga:3306")
        .expect("mysql connection pool");

    rocket::ignite()
        .manage(redis_pool)
        .manage(mysql_pool)
        .mount("/", routes![
        handlers::get_users,
        handlers::add_user,
        handlers::remove_user
        ])
        .launch();
}
