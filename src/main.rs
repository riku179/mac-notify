#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate mac_notify;
extern crate rocket;

use mac_notify::{mac_cap, handlers, redis_client};
use mac_notify::redis_client::User;
use std::thread;
use std::sync::mpsc::Receiver;

fn consumer(ch: Receiver<mac_cap::MacAddr>) -> () {
    match redis_client::get_con("redis://localhost:6379") {
        Ok(con) => loop {
            let mac_addr = ch.recv().unwrap();
            eprintln!("{}", mac_addr);
            if let Ok(user) = User::get_from_redis(&con, mac_addr) {
                User::push_timestamp(&con, &user).unwrap()
            }
        }
        Err(err) => eprintln!("{:?}", err)
    }
}

fn main() {
    thread::spawn(move || mac_cap::start_capture(consumer));

    rocket::ignite()
        .mount("/", routes![handlers::index])
        .launch();
}
