use mac_cap::MacAddr;
use redis::cmd;
use redis::{Commands, RedisResult, Connection, Client};
use chrono;
use chrono::prelude::*;
use chrono::{DateTime, Local};

pub fn get_con(address: &str) -> RedisResult<Connection> {
    let client = Client::open(address)?;
    client.get_connection()
}

#[derive(Debug)]
pub struct User {
    name: String,
    addr: MacAddr,
    timestamp: chrono::DateTime<Local>,
}

impl User {
    pub fn set_into_redis(con: &Connection, name: &String, addr: &MacAddr) -> RedisResult<()> {
        let now = get_now_time();
        set_user_in_redis(con, name, addr, &now)
    }

    pub fn get_from_redis(con: &Connection, addr: MacAddr) -> RedisResult<User> {
        let c: String = con.get(format!("mac:{}", &addr.to_string()))?;
        let (name, timestamp) = parse_redis_context(&c);
        Ok(User {
            name: name,
            addr: addr,
            timestamp: timestamp,
        })
    }

    pub fn push_timestamp(con: &Connection, user: &User) -> RedisResult<()> {
        let now = get_now_time();
        set_user_in_redis(con, &user.name, &user.addr, &now)
    }

    pub fn get_all_list(con: &Connection) -> RedisResult<Vec<User>> {
        let users_str = get_username_list(con)?;
        get_all_users(con, &users_str)
    }
}

fn get_now_time() -> String {
    let time_now = Local::now();
    time_now.format("%Y-%m-%d-%H-%M-%S").to_string()
}

fn get_username_list(con: &Connection) -> RedisResult<Vec<String>> {
    cmd("KEYS").arg("mac:*").query(con)
}

fn get_all_users(con: &Connection, keys: &Vec<String>) -> RedisResult<Vec<User>> {
    let names_str = keys.join(" ");
    let iter = cmd("MGET").arg(names_str).cursor_arg(0).iter(con)?;
    let mut v: Vec<User> = vec![];
    for (c, k) in iter.zip(keys) {
        let (name, timestamp) = parse_redis_context(&c);
        v.push(User {
            name: name,
            addr: MacAddr::from_str(k),
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
        format!("mac:{}", addr.to_string()),
        format!("{}:{}", name, timestamp_str),
    )
}

fn parse_redis_context(str: &String) -> (String, DateTime<Local>) {
    let mut v = str.split(':');
    if let Err(e) = v.nth(1).unwrap().parse::<DateTime<Local>>() {
        eprintln!("{:?}", e)
    }
    (
        v.nth(0).unwrap().to_string(),
        Local
            .datetime_from_str(v.nth(1).unwrap(), "%Y-%m-%d-%H-%M-%S")
            .unwrap(),
    )
}
