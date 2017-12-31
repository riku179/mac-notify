use rocket_state::RedisCon;
use redis_client::User;
use rocket_contrib::Json;
use redis::RedisError;
use mac_cap::MacAddr;

#[derive(Debug, Deserialize)]
struct UserReq {
    pub user_name: String,
    pub mac_addr: MacAddr
}

#[get("/users")]
fn get_users(con: RedisCon) -> Result<Json<Vec<User>>, RedisError> {
    User::get_all_list(&con).map(Json)
}

#[post("/users", format = "application/json", data = "<user>")]
fn add_user(con: RedisCon, user: Json<UserReq>) -> Result<Json<()>, RedisError> {
    User::set_into_redis(&con, &user.user_name, &user.mac_addr).map(Json)
}

#[delete("/users", format = "application/json", data = "<user>")]
fn remove_user(con: RedisCon, user: Json<UserReq>) -> Result<Json<()>, RedisError> {
    User::remove_from_redis(&con, &user.mac_addr).map(Json)
}
