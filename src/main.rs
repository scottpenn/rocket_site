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

#[get("/about")]
fn about() -> Result<NamedFile, NotFound<String>> {
    NamedFile::open("public/pages/about.html").map_err(|n| NotFound(format!("Bad path: {:?}", n)))
}

#[get("/about?<first_name>&<last_name>")]
fn about_replace(first_name: String, last_name: String) -> Result<Html<String>, Error> {

    let mut html = String::new();
    NamedFile::open("public/pages/about.html")?.file_mut().read_to_string(&mut html)?;
    html = html.replace("Scott", &format!("<b>{}</b>", &first_name));
    html = html.replace("Penn", &format!("<b>{}</b>", &last_name));

    Ok(Html(html))
}

//Static file server
#[get("/<file..>", rank = 2)]
fn files(file: PathBuf) -> Result<NamedFile, NotFound<String>> {
    let path = Path::new("public").join(file);
    NamedFile::open(&path).map_err(|_| NotFound(format!("Bad path: {:?}", path)))
}

fn main() {
    rocket::ignite().mount("/", routes![index, about, about_replace, files]).mount("/", StaticFiles::from("public")).launch();
}