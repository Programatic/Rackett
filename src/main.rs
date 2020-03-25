/*mod index;

use index::{EntertainmentType, PirateBay, Indexer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let p = PirateBay::new();
    let t = p.search(EntertainmentType::MOVIE, "kingsman").await?;
    println!("{:#?}", t);

    Ok(())
}*/

mod omdb;
mod index;

#[macro_use]
extern crate lazy_static;

use actix_web::{http, web, App, HttpServer, Responder, HttpResponse, get, post, Error as AWError};
use actix_files as fs;
use serde::Deserialize;
use omdb::{OMDB as OMDBApi};
use index::{EntertainmentType, PirateBay, Indexer};

lazy_static! {
    static ref OMDB: OMDBApi = OMDBApi::new("1775f8d1");
}

#[derive(Deserialize)]
pub struct SearchData {
    search: String,
    entertainment_type: String
}

#[derive(Deserialize)]
struct IndexerSearch {
    search: String
}

#[post("/search")]
async fn search(form: web::Form<SearchData>) -> Result<impl Responder, AWError> {
    let result = OMDB.search(&form).await.unwrap();
    Ok(HttpResponse::Ok()
        .set_header(http::header::CONTENT_TYPE, "application/json")
       .body(result))
}

#[post("/indexers/search")]
async fn indexers_search(form: web::Form<IndexerSearch>) -> Result<impl Responder, AWError> {
    // TODO: Update to search active vector
    let p = PirateBay::new();
    let t = p.search(EntertainmentType::MOVIE, &form.search).await.unwrap();
    let mut ret = String::from("{\"torrents\": [");
    for (i, torrent) in t.iter().enumerate() {
        ret = ret + &serde_json::to_string(&torrent)?;
        if i != t.len() - 1 {
            ret = ret + ","
        }
    }
    ret = ret + "]}";
    Ok(HttpResponse::Ok()
        .set_header(http::header::CONTENT_TYPE, "application/json")
        .body(ret))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(search)
            .service(indexers_search)
            //.service(fs::Files::new("/", "frontend").show_files_listing())
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
