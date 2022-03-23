use mysql::*;
use mysql::prelude::*;

pub fn main() {
    let url = "mysql://root:yt66@localhost:3307/MySql80";
    let pool = Pool::new(url);
    let mut conn = pool.get_conn();
}
