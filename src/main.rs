#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
extern crate rocket_contrib;

#[macro_use]
extern crate lazy_static;

use std::fs::File;
use std::fs::read_to_string;
use std::io;
use std::io::Error;
use std::path::{PathBuf, Path};
use rocket::response::NamedFile;
use rocket::response::status::NotFound;
use rocket::response::content::Html;
use rocket_contrib::serve::StaticFiles;

lazy_static! {
    static ref HEAD: String = {
        read_to_string("public/pages/head.html").expect("No file named head.html")
    };
    static ref NAV: String = {
        read_to_string("public/pages/nav.html").expect("No file named nav.html")
    };

}

fn construct_page(filename: &str) -> String {
    let mut index = read_to_string(filename).expect(&format!("No file named {}.", filename));
    index = index.replace("!head!", &*HEAD);
    index = index.replace("!nav!", &*NAV);
    index
}

#[get("/")]
fn index() -> Html<&'static str> {

    lazy_static! {
        static ref INDEX: String = {
            construct_page("public/pages/index.html")
        };
    }

    Html(&*INDEX)
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