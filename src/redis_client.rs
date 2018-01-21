use mac_cap::MacAddr;
use redis::cmd;
use redis::{Commands, RedisResult, Connection, Client};
use chrono;
use chrono::prelude::*;
use chrono::{DateTime, Local};

pub fn get_con(address: &str) -> RedisResult<Connection> {
    let client = Client::open(address)?; client.get_connection()
}

#[derive(Debug, Serialize)]
pub struct User {
    pub name: String,
    pub addr: MacAddr,
    timestamp: chrono::DateTime<Local>,
}

impl User {
    pub fn set_into_redis(con: &Connection, name: &String, addr: &MacAddr) -> RedisResult<()> {
        let now = get_now_time();
        set_user_in_redis(con, name, addr, &now)
    }

    pub fn get_from_redis(con: &Connection, addr: MacAddr) -> Option<User> {
        let c = con.get(get_mac_key(&addr)).ok()?;
        let (name, timestamp) = parse_redis_context(&c);
        Some(User {
            name,
            addr: addr,
            timestamp: timestamp,
        })
    }

    pub fn remove_from_redis(con: &Connection, addr: &MacAddr) -> RedisResult<()> {
        con.del(get_mac_key(&addr))
    }

    pub fn push_timestamp(con: &Connection, user: &User) -> RedisResult<()> {
        let now = get_now_time();
        set_user_in_redis(con, &user.name, &user.addr, &now)
    }

    pub fn get_all_list(con: &Connection) -> RedisResult<Vec<User>> {
        let keys = get_keys(con)?;
        get_all_users(con, keys)
    }
}

fn get_mac_key(addr: &MacAddr) -> &str {
    format!("mac:{}", addr.to_string())
}

fn get_now_time() -> String {
    let time_now = Local::now();
    time_now.format("%Y-%m-%d-%H-%M-%S").to_string()
}

fn get_keys(con: &Connection) -> RedisResult<Vec<String>> {
    cmd("KEYS").arg("mac:*").query(con)
}

fn get_all_users(con: &Connection, keys: Vec<String>) -> RedisResult<Vec<User>> {
    let values = cmd("MGET").arg(keys.clone()).cursor_arg(0).iter(con)?;
    let mut v: Vec<User> = vec![];

    for (value, k) in values.zip(keys) {
        let (name, timestamp) = parse_redis_context(&value);
        let mac_addr = k.split(':').nth(1).expect("failed to parse redis record").to_string();
        v.push(User {
            name: name,
            addr: mac_addr.parse().expect("failed to parse Mac address"),
            timestamp: timestamp,
        })
    }
    Ok(v)
}

fn set_user_in_redis(
    con: &Connection,
    name: &String,
    addr: &MacAddr,
    timestamp_str: &String,
) -> RedisResult<()> {
    con.set(
        get_mac_key(&addr),
        format!("{}:{}", name, timestamp_str),
    )
}

fn parse_redis_context(str: &String) -> (String, DateTime<Local>) {
    let v: Vec<&str> = str.split(':').collect();
    match Local.datetime_from_str(&v[1].to_string(), "%Y-%m-%d-%H-%M-%S",) {
        Ok(date_time) => (v[0].to_string(), date_time),
        Err(e) => panic!(e)
    }
}
