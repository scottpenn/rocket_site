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

//common elements
lazy_static! {
    static ref HEAD: String = {
        read_to_string("public/pages/partials/head.html").expect("No file named head.html")
    };
    static ref NAV: String = {
        read_to_string("public/pages/partials/nav.html").expect("No file named nav.html")
    };

}

//page content
lazy_static! {
    static ref INDEX: String = {
        construct_page("public/pages/index.html")
    };
    static ref ABOUT: String = {
        construct_page("public/pages/about.html")
    };
    static ref EULER: String = {
        construct_page("public/pages/euler.html")
    };
    static ref TWITTER: String = {
        construct_page("public/pages/twitter.html")
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
    Html(&*INDEX)
}

#[get("/euler")]
fn euler() -> Html<&'static str> {
    Html(&*EULER)
}

#[get("/about")]
fn about() -> Html<&'static str> {
    Html(&*ABOUT)
}

#[get("/twitter")]
fn twitter() -> Html<&'static str> {
    Html(&*TWITTER)
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