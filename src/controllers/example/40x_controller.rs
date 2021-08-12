use actix_web::{get, post, put, patch, delete, head, guard, web, App, HttpRequest, HttpResponse, HttpServer, Responder, error, Error};
use serde::{Serialize};
use std::sync::Mutex;
use tera::Tera;

async fn 404(tpl: web::Data<Tera>) -> Result<HttpResponse, Error>{
    let mut ctx = tera::Context::new();
    let view = tpl.render("example/40x/404.html.tera", &ctx).map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(view))
}

