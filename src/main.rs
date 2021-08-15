use actix_web::{get, post, put, patch, delete, guard, web, App, HttpRequest, HttpResponse, HttpServer, Responder, error, Result};
use actix_web::http::StatusCode;
//use actix_web::middleware::errhandlers::{ErrorHandlerResponse, ErrorHandlers};
use actix_web::middleware::Logger;
use actix_session::{CookieSession};
use env_logger::Env;
use serde::{Serialize};
use states::{app as app_state , example as ex_state};
use std::sync::{Arc, Mutex};
use derive_more::{Display, Error};
use tera::{Tera};
use rand::Rng;

mod routes;
mod states;
mod controllers;
use controllers::example::{default_controller as ex_default};


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    //ロガー
    env_logger::from_env(Env::default().default_filter_or("info")).init();

    //--------------------------------
    //共有メモリ(不変): shared immutable state
    //  -> Cloneを継承したstructを使えば .data( ex_app.clone() )という呼び方も出来るが
    //  -> コンパイラーに「コスト掛かるぜ、安全だけどな」って警告される、今は見易さ優先で使う
    //  -> ※起動時に少しだけコスト掛かるのと少量のメモリ占有だけだと思うんだけど
    //--------------------------------
    let ex_app = ex_state::AppState{
        app_name: "headress".to_string(),
        author: "masataka yamamoto".to_string()
    };
    //  ~~ .data(ex_app.clone())
    //--------------------------------
    
    //--------------------------------
    //サーバー間共有メモリ(可変): shared immutable state
    //Share state Example : 直書きしか出来ないのでここに書く
    //--------------------------------
    let mut_shared_state = web::Data::new(ex_state::MutableMessageState{
        text: Mutex::new("default string".to_string())
    });

    //--------------------------------
    //Tera(テンプレートエンジン)
    //--------------------------------
    let tera = Tera::new("src/resources/templates/**/*.html.tera").unwrap();
    //println!("{:?}", tera);
    //let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();
    /*
    let templates = match Tera::new("resources/templates/**/*.tera") {
        Ok(tr) =>  tr,
        Err(e) => {
            println!("Tera template parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };
    */
    
    HttpServer::new(move || { 
        
        //--------------------------------
        //サーバー毎の共有メモリ(固定): shared immutable state
        //mutexとか何も考えなくていい
        //--------------------------------
        let mut rng = rand::thread_rng();
        let random: usize = rng.gen();
        let immutable_int_state = ex_state::ImmutableIntState{random_uint: random};
        println!("rand: {}", &immutable_int_state.random_uint);

        //--------------------------------
        //サーバー毎の共有メモリ(可変): shared immutable state by server
        //--------------------------------
        let mut_shared_servers_name = web::Data::new(Mutex::new("default server name".to_string()));
    
        //********START APP********//
        App::new()

        //ロギング
        .wrap(Logger::default())
        .wrap(Logger::new("%a %{User-Agent}i"))
        
        //----------------------
        //  test
        //----------------------
        .route("/", web::get().to(||{HttpResponse::Ok().content_type("text/plain; charset=UTF-8").body("うごいてまーす") } ))

        //----------------------
        //  example
        //----------------------
        .configure(routes::web::example_config) //Remark: routes/example.rs
        //.data(Arc::clone(&arc_ex_app_state))

        //不変の共有メモリ(shared immutable state)
        //.data(ex_state::AppState{
        //    author: "masataka yamamoto".to_string(),
        //    app_name: "headresscms:ヘッドレスCMS".to_string()
        //})
        .data(ex_app.clone())
        //可変の共有メモリ(shared mutable state)
        .app_data(mut_shared_state.clone())
        //サーバー毎の共有メモリ
        .data(immutable_int_state)
        .app_data(mut_shared_servers_name.clone())
        .configure(example_state::example_config)
        
        //セッション使えるように、しかもクッキーセッションという謎技術
        //(web.rsとかのconfigu配下だとrouteを指定しないといけない（？）から不便)//どうせ共通だからここに書けばいい
        .wrap(
            CookieSession::signed(&[0; 32]) // <- create cookie based session middleware
                .secure(false),
        )    

        //teraを各ルートから使えるように
        //.data(templates)
        .data(tera.clone())
        
        //40xなどのデフォルト※ルート毎に設定するのでweb.rsとかに書いた方がいいかも
        //.default_service(web::route().to(ex_default::_404))
 

        //----------------------
        //  main
        //----------------------
        //.configure(web_config)

    })
    .workers(10)
    .bind("0.0.0.0:80")?
    .run()
    .await
}

//----------------------
//State route sample
//----------------------
mod example_state{
    use super::*;

    pub fn example_config(cfg: &mut web::ServiceConfig) {
        cfg.service(
        web::scope("/example_top")
        .route("/app_state", web::get().to(example_app_state))
        .route("/mut_state", web::get().to(example_mut_state_check))
        .route("/mut_state/{setval}", web::get().to(example_mut_state))
        .route("/mut_state_2}", web::get().to(example_mut_state_2))
        .route("/mut_state_servers_name", web::get().to(example_mut_state_servers_name))
        .route("/mut_state_servers_name/{newname}", web::get().to(example_mut_state_set_servers_name))
        );
    }
    
    async fn example_app_state(data: web::Data<ex_state::AppState>) -> HttpResponse{
        std::thread::sleep(std::time::Duration::from_secs(4));
        HttpResponse::Ok()
        .content_type("text/plain; charset=UTF-8")
        .body( format!("app_name:{}, author:{}", data.app_name, data.author) )
    }

    async fn example_mut_state_check(state: web::Data<ex_state::MutableMessageState>) -> HttpResponse{
        HttpResponse::Ok()
        .content_type("text/plain; charset=UTF-8")
        .body(format!("{}", state.text.lock().unwrap()))
    }

    async fn example_mut_state(
        web::Path(setval): web::Path<String>,
        state: web::Data<ex_state::MutableMessageState>
    ) -> HttpResponse{
        let mut text = state.text.lock().unwrap();
        eprint!("setval:{}", &setval);
        *text = setval;

        //std::thread::sleep(std::time::Duration::from_secs(4));
        HttpResponse::Ok()
        .content_type("text/plain; charset=UTF-8")
        .body( format!("mut_text:{}", text) )
    }

    async fn example_mut_state_2(data: web::Data<ex_state::ImmutableIntState>) -> HttpResponse{
        std::thread::sleep(std::time::Duration::from_secs(5));
        HttpResponse::Ok()
        .content_type("text/plain; charset=UTF-8")
        .body(format!("{}", data.random_uint))
    }

    async fn example_mut_state_servers_name(data: web::Data<Mutex<String>>) -> HttpResponse{
        std::thread::sleep(std::time::Duration::from_secs(3));
        HttpResponse::Ok()
        .content_type("text/plain; charset=UTF-8")
        .body(format!("{}", data.lock().unwrap()))
    }

    async fn example_mut_state_set_servers_name(
        web::Path(newname): web::Path<String>, 
        data: web::Data<Mutex<String>>
    ) -> HttpResponse{
        let mut servername = data.lock().unwrap();
        *servername = newname;
        HttpResponse::Ok()
        .content_type("text/plain; charset=UTF-8")
        .body(format!("Set new server_name"))
    }



}
