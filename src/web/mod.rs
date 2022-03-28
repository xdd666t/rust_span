use actix_web::{App, HttpServer};

use crate::service::CONTEXT;

mod web_api;

pub async fn init_web() -> std::io::Result<()> {
    let route = || {
        App::new()
            .service(web_api::index)
            .service(web_api::add)
    };

    HttpServer::new(route)
        .bind(&CONTEXT.config.server_url)?
        .run()
        .await
}




