extern crate mac_notify;
extern crate pcap;

use mac_notify::mac_cap;

fn main() {
    mac_cap::start_capture();
}
