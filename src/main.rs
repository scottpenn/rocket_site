#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
extern crate rocket_contrib;

use std::io::Read;
use std::io::Error;
use std::path::{PathBuf, Path};
use rocket::response::NamedFile;
use rocket::response::status::NotFound;
use rocket::response::content::Html;
use rocket_contrib::serve::StaticFiles;

#[get("/")]
fn index() -> Result<NamedFile, NotFound<String>> {
    NamedFile::open("public/pages/index.html").map_err(|n| NotFound(format!("Bad path: {:?}", n)))
}

#[get("/euler")]
fn euler() -> Result<NamedFile, NotFound<String>> {
    NamedFile::open("public/pages/euler.html").map_err(|n| NotFound(format!("Bad path: {:?}", n)))
}

#[get("/about")]
fn about() -> Result<NamedFile, NotFound<String>> {
    NamedFile::open("public/pages/about.html").map_err(|n| NotFound(format!("Bad path: {:?}", n)))
}

#[get("/twitter")]
fn twitter() -> Result<NamedFile, NotFound<String>> {
    NamedFile::open("public/pages/twitter.html").map_err(|n| NotFound(format!("Bad path: {:?}", n)))
}

//Static file server
#[get("/<file..>", rank = 2)]
fn files(file: PathBuf) -> Result<NamedFile, NotFound<String>> {
    let path = Path::new("public").join(file);
    NamedFile::open(&path).map_err(|_| NotFound(format!("Bad path: {:?}", path)))
}

fn main() {
    rocket::ignite().mount("/", routes![index, euler, about, twitter, files]).mount("/", StaticFiles::from("public")).launch();
}