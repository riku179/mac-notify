use std::{env, thread, fmt};
use pcap::{self, Device, Capture, Active};
use std::sync::mpsc::{channel, Receiver};
use std::io::{Cursor, Seek, SeekFrom, Read};

#[derive(Debug)]
pub struct MacAddr([u8; 3]);

impl MacAddr {
    pub fn new(arg: [u8; 3]) -> MacAddr {
        MacAddr(arg)
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
    println!("{}", main_device.name);

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

fn consumer(ch: Receiver<MacAddr>) {
    loop {
        let mac_addr = ch.recv().unwrap();
        println!("{:?}", mac_addr);
    }
}

pub fn start_capture() -> () {
    match get_capture(get_interface()) {
        Ok(c) => {
            let mut buf: [u8; 3] = [0; 3];
            let (tx, rx) = channel();
            thread::spawn(move || consumer(rx));

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