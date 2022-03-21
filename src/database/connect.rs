use mysql::*;
use mysql::prelude::*;
use chrono::prelude::*; // 用来处理日期

pub fn main() {
    let url = "mysql://root:password@localhost:3306/MYDB";
    let pool = Pool::new(url).unwrap(); // 获取连接池
    let mut connect = pool.get_conn().unwrap();// 获取链接
}