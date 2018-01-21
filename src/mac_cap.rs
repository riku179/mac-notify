use std::{env, thread, fmt, str, num};
use pcap::{self, Device, Capture, Active};
use std::sync::mpsc::{channel, Receiver};
use std::io::{Cursor, Seek, SeekFrom, Read};
use redis::Connection;
use serde::{de, Serialize, Deserialize, Serializer, Deserializer};

#[derive(Debug)]
pub struct MacAddr([u8; 3]);

impl Serialize for MacAddr {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for MacAddr {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct MacAddrVisitor;
        impl<'de> de::Visitor<'de> for MacAddrVisitor {
            type Value = MacAddr;

            fn visit_str<E: de::Error>(self, value: &str) -> Result<MacAddr, E> {
                value
                    .parse()
                    .map_err(|err| E::custom(&format!("{}", err)))
            }

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "a string representation of a MAC address")
            }
        }
        deserializer.deserialize_str(MacAddrVisitor)
    }
}

impl MacAddr {
    pub fn new(arg: [u8; 3]) -> MacAddr { MacAddr(arg) }
}

impl str::FromStr for MacAddr {
    type Err = num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut v: [u8; 3] = [0; 3];
        for (i, x) in s.split('-').enumerate() {
            v[i] = x.parse::<u8>()?
        }
        Ok(MacAddr(v))
    }
}

impl fmt::Display for MacAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}-{}-{}", self.0[0], self.0[1], self.0[2])
    }
}

fn get_capture(device_name: String) -> Result<Capture<Active>, pcap::Error> {
    let main_device = Device {
        name: device_name,
        desc: None,
    };
    println!("captured device: {}", main_device.name);

    Capture::from_device(main_device)
        .unwrap()
        .promisc(true)
        .snaplen(10)
        .open()
}

fn get_interface() -> String {
    match env::args().nth(1) {
        Some(nic) => nic,
        None => "eno1".to_string(),
    }
}

pub fn start_capture(consumer: fn(Receiver<MacAddr>, Connection) -> (), con: Connection) -> () {
    match get_capture(get_interface()) {
        Ok(c) => {
            let mut buf: [u8; 3] = [0; 3];
            let (tx, rx) = channel();
            thread::spawn(move || consumer(rx, con));

            let mut cap = c;

            while let Ok(packet) = cap.next() {
                let mut cur = Cursor::new(packet.data);
                if let Err(_) = cur.seek(SeekFrom::Start(6)) {
                    print!("[ERROR] cannot seek packet!");
                    break;
                }
                cur.read(&mut buf).unwrap();
                tx.send(MacAddr::new(buf)).unwrap();
            }
        }
        Err(err) => print!("{:?}\n", err),
    }
}