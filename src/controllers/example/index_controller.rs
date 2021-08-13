use actix_web::{get, post, put, patch, delete, head, guard, web, App, HttpRequest, HttpResponse, HttpServer, Responder, error, Result};
use serde::{Serialize};
use std::sync::Mutex;
use derive_more::{Display, Error};

pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, world !")
}

pub async fn contenttype() -> impl Responder {
    HttpResponse::Ok()
    .content_type("text/plain")
    .body("ContentType text/plain")
}

pub async fn pathparameter(web::Path((user_no, page)): web::Path<(usize, String)>) -> Result<String> {
    Ok(format!("/pathparameter/{}/{}", user_no, page))
}

pub async fn _get() -> Result<String> {
    Ok(format!("get"))
}

pub async fn _post() -> Result<String> {
    Ok(format!("post"))
}

pub async fn _put() -> Result<String> {
    Ok(format!("put"))
}

pub async fn _patch() -> Result<String> {
    Ok(format!("patch"))
}

pub async fn _delete() -> Result<String> {
    Ok(format!("delete"))
}

pub async fn _head() -> Result<String> {
    Ok(format!("head"))
}


