use std::ops::Deref;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};
use r2d2_redis::RedisConnectionManager;
use r2d2::{Pool, PooledConnection};

// ref) https://rocket.rs/guide/state/#connection-guard

pub struct RedisCon(pub PooledConnection<RedisConnectionManager>);

impl<'a, 'r> FromRequest<'a, 'r> for RedisCon {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<RedisCon, ()> {
        let pool = request.guard::<State<Pool<RedisConnectionManager>>>()?;
        match pool.get() {
            Ok(con) => Outcome::Success(RedisCon(con)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ()))
        }
    }
}

impl Deref for RedisCon {
    type Target = PooledConnection<RedisConnectionManager>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
