use actix_web::{get, post, put, patch, delete, guard, web, App, HttpRequest, HttpResponse, HttpServer, Responder, error, Result};
//use actix_rt;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use derive_more::{Display, Error};

#[derive(Serialize)]
struct JsonObj{
    str: String,
    num: isize,
    arr: Vec::<String>
}

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


//TEST
#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http, test};
    #[actix_rt::test]
    async fn test_json_response_ok() {
        let req = test::TestRequest::with_header("content-type", "text/plain").to_http_request();
        let resp = json_response(req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);
    }

/*
    #[actix_rt::test]
    async fn test_index_not_ok() {
        let req = test::TestRequest::default().to_http_request();
        let resp = json_response(req).await;
        assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);
    }
*/
}