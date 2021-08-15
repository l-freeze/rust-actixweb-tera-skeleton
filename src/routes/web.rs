use actix_web::{get, post, put, patch, delete, guard, web, App, HttpRequest, HttpResponse, HttpServer, Responder, error, Result};
//use actix_session::{CookieSession, Session};
use crate::controllers::example::{
    index_controller as ex_idx, 
    json_controller as ex_json, 
    html_controller as ex_html, 
    default_controller as ex_default,
    session_controller as ex_session,
    static_controller as ex_static

};
//mod controllers::{example};
//use example::{index_controller as ex_idx, json_controller as ex_json, html_controller as ex_html, default_controller as ex_default};
 
//動作確認用ルーティング
pub fn example_config(cfg: &mut web::ServiceConfig) {
    
    //Example
    //Test routing1
    //curl.exe -X [GET|POST] -H 'OriginalHead: org' localhost/testroute1
    cfg.service(
        web::scope("/example/testroute1")  //http://localhost/example/testroute1
        .route("", web::to(||HttpResponse::Ok().body("testroute1")))
    );

    //Test routing2
    cfg.service(
        web::scope("/example/testroute2") //http://localhost/example/testroute2
        .route("", web::get().to(||HttpResponse::Ok().body("testroute2:get")))
        .route("", web::post().to(||HttpResponse::Ok().body("testroute2:post")))
        .route("", web::head().to(||HttpResponse::Ok().body("testroute2:head")))
        //curl.exe -X [GET|POST] -H 'OriginalHead: org' localhost/testroute1
    );


    cfg.service(
        web::scope("/example") //http://localhost/example/***
        //index_controller
        .route("/", web::get().to(ex_idx::index))
        .route("/contenttype", web::get().to(ex_idx::contenttype))
        .route("/pathparameter/{user_no}/{page}", web::get().to(ex_idx::pathparameter))
        .route("/resource", web::get().to(ex_idx::_get))
        .route("/resource", web::post().to(ex_idx::_post))
        .route("/resource", web::put().to(ex_idx::_put))
        .route("/resource", web::patch().to(ex_idx::_patch))
        .route("/resource", web::delete().to(ex_idx::_delete))
        .route("/resource", web::head().to(ex_idx::_head))

        //json_controller
        .route("/json_get", web::get().to(ex_json::json_response))
            //curl.exe -v -X POST -H 'AddHeader: addh'  -d '{"username":"lfreeze", "freeword":"日本語"}' localhost/example/json_post
        .route("/json_post", web::post().to(ex_json::json_post))
        //Linux -> curl.exe -v -X POST -H 'application/json'  -d '{"username":"lfreeze", "freeword":"日本語"}' localhost/example/json_post
        //Win   -> curl.exe -X POST -H "Content-Type: application/json" -d '{\"username\":\"lfreeze\", \"freeword\":\"cant send mulitbyte\"}' localhost/example/json_post
        //         curl.exe -i -X POST -H "Content-Type: application/json" -d '{\"username\":\"hello\", \"freeword\":\"hello\"}' localhost/example/json_post
        .route("/html", web::get().to(ex_html::html))
        
        //セッション管理
        .route("/session", web::get().to(ex_session::index))

        //デフォルトページ
        .default_service(web::route().to(ex_default::_404))

        //static file route
        .route("/js/{filepath}", web::get().to(ex_static::static_file))
        .route("/css/{filepath}", web::get().to(ex_static::static_file))
        .route("/img/{filepath}", web::get().to(ex_static::static_file))
    );

    //cfg.default_service(web::route().to(ex_default::_404));

}

//Admin
pub fn admin_config(cfg: &mut web::ServiceConfig) {
    //Example
    cfg.service(
        //AdminTop
        web::scope("/")
        .guard(guard::Header("Roll", "admin"))
        .route("/", web::to(||HttpResponse::Ok().body("body")))
    );
    
    cfg.service(
        //UserTop
        web::scope("/")
        .route("/", web::to(||HttpResponse::Ok().body("body")))
        //.route("/", web::to(inscope1))
    );    
}

//User
pub fn user_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/")
        .route("/", web::to(||HttpResponse::Ok().body("body")))
    );
    
    cfg.service(
        web::scope("/")
        .route("/", web::to(||HttpResponse::Ok().body("body")))
        //.route("/", web::to(inscope1))
    );    
}
