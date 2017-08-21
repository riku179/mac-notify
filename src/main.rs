#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate mac_notify;
extern crate pcap;
extern crate rocket;

use mac_notify::{mac_cap, handlers};
use std::thread;


fn main() {
    thread::spawn(move || mac_cap::start_capture());

    rocket::ignite()
       .mount("/", routes![handlers::index])
       .launch();
}
