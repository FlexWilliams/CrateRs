#[macro_use] extern crate rocket;

use std::path::PathBuf;

#[get("/")]
fn index() -> &'static str {
    "Hello, from CrateRs!\n"
}

#[head("/<path..>")]
fn head(path: PathBuf) -> String {
    println!("path: {}", path.display());
    format!("HEAD: {}\n", path.display())
}

#[get("/<path..>")]
fn get(path: PathBuf) -> String {
    println!("path: {}", path.display());
    format!("GET: {}\n", path.display())
}

#[put("/<path..>")]
fn put(path: PathBuf) -> String {
    println!("path: {}", path.display());
    format!("PUT: {}\n", path.display())
}

#[delete("/<path..>")]
fn delete(path: PathBuf) -> String {
    println!("path: {}", path.display());
    format!("DELETE: {}\n", path.display())
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/storage", routes![head, get, put, delete])
}