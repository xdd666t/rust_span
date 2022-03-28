use actix_web::{HttpResponse, Responder};
use rbatis::crud::CRUD;

use crate::data::base_info::BaseResult;
use crate::service::CONTEXT;

#[get("/")]
pub async fn index() -> impl Responder {
    let info = BaseResult {
        code: String::from("222222222"),
        data: String::from("333333333"),
        success: true,
    };
    let msg = serde_json::to_string(&info).unwrap();
    HttpResponse::Ok().body(msg)
}

#[get("/add")]
pub async fn add() -> impl Responder {
    let info = BaseResult {
        code: String::from("200"),
        data: String::from("新增成功！"),
        success: true,
    };

    let result = CONTEXT.rb.save(&info, &[]).await;
    if let Ok(msg) = result {
        return HttpResponse::Ok().body("新增成功");
    }

    if let Err(e) = result {
        return HttpResponse::Ok().body("添加失败：".to_owned() + &e.to_string());
    }

    HttpResponse::Ok().body("未知结果")
}