use std::fmt;
use mac_cap::MacAddr;
use redis;
use redis::{Commands, RedisResult};
use chrono::Local;

pub struct User {
    name: String,
    addr: MacAddr,
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.name, self.addr)
    }
}

impl User {
    pub fn set_redis(user: User, con: redis::Connection) -> RedisResult<()> {
        let time_now = Local::now();
        let now_str = time_now.format("%Y-%m-%d-%H-%M-%S");

        let _: () = try!(con.set(user.to_string(), now_str.to_string()));
        Ok(())
    }
}
