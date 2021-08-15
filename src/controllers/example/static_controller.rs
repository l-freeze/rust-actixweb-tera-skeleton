use actix_files::NamedFile;
use actix_web::{HttpRequest, Result, dev::Path, web};
use std::path::PathBuf;
use serde::Deserialize;


#[derive(Deserialize, Debug)]
pub struct QueryString {
    filename: String,
}

//http://localhost/example/img/genbaneko1.jpg?filename=/assets/img/genbaneko1.jpg
pub async fn static_file(req: HttpRequest, 
    web::Path(filepath): web::Path<String>, 
    query: web::Query<QueryString>
) -> Result<NamedFile> {
    println!("Request : fn static_file");
    println!("{}", filepath);
    //let path: PathBuf = req.match_info().query("filepath").parse().unwrap();
    //println!("{}", path.to_string_lossy().to_string());
    //let q = req.query_string();
    //println!("{:?}", q);
    let path: PathBuf = PathBuf::from(&query.filename);
    //path PathBuf = PathBuf::from("&query.filename");
    println!("{:?}", query.filename);
    println!("{:?}", path);
    //Ok(NamedFile::open(path)?)
    Ok(NamedFile::open(path)?)
}

