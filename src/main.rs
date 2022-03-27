use rust_web::service::CONTEXT;
use rust_web::web;

#[tokio::main]
async fn main() -> std::io::Result<()>{
    //连接数据
    CONTEXT.link_db().await;

    //初始化web
    web::init_web().await
}