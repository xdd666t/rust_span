use actix_web::{HttpResponse, Responder};
use crate::data::base_info::BaseResult;


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