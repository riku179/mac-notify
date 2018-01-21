use diesel;
use diesel::mysql::MysqlConnection;
use diesel::r2d2;
use diesel::r2d2::ConnectionManager;

type Pool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

pub fn new_mysql_pool(url: &str) -> Result<Pool, r2d2::PoolError> {
    let manager = ConnectionManager::<MysqlConnection>::new(url);
    r2d2::Pool::new(manager)
}