use actix_web::{get, post, put, patch, delete, guard, web, App, HttpRequest, HttpResponse, HttpServer, Responder, error, Result};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use derive_more::{Display, Error};

#[derive(Serialize)]
struct JsonObj{
    str: String,
    num: isize,
    arr: Vec::<String>
}

#[get("/json_get")]
pub async fn json_response() -> HttpResponse {
    HttpResponse::Ok().json(
        JsonObj{
            str: "サンプル文字列".to_string(),
            num: 123456789,
            arr: vec!["String".to_string(), "ストリング".to_string(), "すとりんぐ".to_string(), ]
        }
    )
}


#[derive(Deserialize)]
pub struct SampleObj {
    username: String,
    freeword: String,
}


#[post("/json_post")]
pub async fn json_post(req_json: Option<web::Json<SampleObj>>) -> HttpResponse {
    if let Some(json) = req_json {
        HttpResponse::Ok()
        .content_type("application/json")
        //.body(format!("username:{}, freeword:{}", req_json.username, req_json.freeword))
        .body(format!("username:{}, freeword:{}", json.username, json.freeword))
    } else {
        HttpResponse::Ok()
        .content_type("application/json")
        .body("No request data")        

    }
}
