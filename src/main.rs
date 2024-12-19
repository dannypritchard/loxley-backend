#[macro_use]
extern crate rocket;
use std::vec;

use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::serde::{json::Json, Serialize};
use rocket::{Request, Response};

pub struct Cors;

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Cross-Origin-Resource-Sharing Fairing",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, PATCH, PUT, DELETE, HEAD, OPTIONS, GET",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Block {
    title: String,
    content: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Content {
    title: String,
    subtitle: String,
    content: Vec<Block>,
}

#[get("/")]
async fn index() -> Json<Content> {
    Json(Content {
        title: "Loxley Industries".to_owned(),
        subtitle: "Precision crafted software".to_owned(),
        content: vec![Block {
            title: "title".to_owned(),
            content: "content".to_owned(),
        }],
    })
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .attach(Cors)
        .mount("/", routes![index])
        .launch()
        .await?;

    Ok(())
}
