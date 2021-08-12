use actix_web::{get, post, put, patch, delete, guard, web, App, HttpRequest, HttpResponse, HttpServer, Responder, error, Result};

use crate::controllers::example::{index_controller as ex_idx, json_controller as ex_json, html_controller as ex_html, default_controller as ex_default};
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
      .service(ex_idx::index)// #[get("/")]
      .service(ex_idx::contenttype)// #[get("/contenttype")]
      .service(ex_idx::pathparameter)// #[get("/pathparameter/{user_no}/{page}")]
      .service(ex_idx::pathparameter)// #[get("/pathparameter/{user_no}/{page}")]
      .service(ex_idx::_get)// #[ ***("/resource")]
      .service(ex_idx::_post)// #[ ***("/resource")]
      .service(ex_idx::_put)// #[ ***("/resource")]
      .service(ex_idx::_patch)// #[ ***("/resource")]
      .service(ex_idx::_delete)// #[ ***("/resource")]
      .service(ex_idx::_head)// #[ ***("/resource")]

      //json_controller
      .service(ex_json::json_response)// #[get("/json_get")]
        //curl.exe -v -X POST -H 'AddHeader: addh'  -d '{"username":"lfreeze", "freeword":"日本語"}' localhost/example/json_post
        .service(ex_json::json_post)// #[post("/json_post")]
        //Linux -> curl.exe -v -X POST -H 'application/json'  -d '{"username":"lfreeze", "freeword":"日本語"}' localhost/example/json_post
        //Win   -> curl.exe -X POST -H "Content-Type: application/json" -d '{\"username\":\"lfreeze\", \"freeword\":\"cant send mulitbyte\"}' localhost/example/json_post
        //         curl.exe -i -X POST -H "Content-Type: application/json" -d '{\"username\":\"hello\", \"freeword\":\"hello\"}' localhost/example/json_post
       .service(ex_html::html) // https://localhost/example/html
       .default_service(web::route().to(ex_default::_404))
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
