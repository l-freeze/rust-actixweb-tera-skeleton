use actix_web::{get, post, put, patch, delete, head, guard, web, App, HttpRequest, HttpResponse, HttpServer, Responder, error, Error, Result};
use actix_web::http::{StatusCode};
use serde::{Serialize};
use std::sync::Mutex;
use tera::Tera;

pub async fn _404() -> Result<HttpResponse> {
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body("<h1>Error 404</h1>"))
}


