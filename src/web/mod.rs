use actix_web::{App, HttpServer};
use rbatis::crud::CRUD;
use crate::service::CONTEXT;

mod web_api;

pub async fn init_web() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web_api::index)
    })
        .bind(&CONTEXT.config.server_url)?
        .run()
        .await
}

fn test() {
    CONTEXT.r_batis.save();
}


