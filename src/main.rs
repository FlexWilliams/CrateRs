#[macro_use] extern crate rocket;

use std::path::{PathBuf, Path};
use rocket::http::{Status, ContentType};


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
fn put(path: PathBuf) -> (Status, (ContentType, &'static str)) {
    match path.to_str() {
        Some(document_path) => {
            let _path = format!("{}/{}", "username", document_path);
            let storage_path = Path::new(&_path);

            if storage_path.is_dir() {
                print!("The path is a directory: {document_path}");
                return (Status::BadRequest, (ContentType::Text, "The specified path is a directory! Unable to proceed.\n"));
            } 

            if storage_path.is_file() {
                let x = storage_path.display();
                println!("The path is to a file: {x}");
                return (Status::Created, (ContentType::Text, "File created!\n"))
            }

            // TODO: handle file creation!
            return (Status::Ok, (ContentType::Text, "File not yet created!\n"))
        },
        None => {
            // REVIEW: Looks to be unreachable code?
            // cURLing routes `/storage` & `/storage/` & `/storage///` yield a Some value (of empty string)
            println!("path is error"); // TODO: logger.error this
            return (Status::InternalServerError, (ContentType::Text, "Error Occurred reading path."));
        },
    }
}

#[delete("/<path..>")]
fn delete(path: PathBuf) -> (Status, (ContentType, &'static str)) {
    match path.to_str() {
        Some(document_path) => {
            let _path = format!("{}/{}", "username", document_path);
            let storage_path = Path::new(&_path);

            if storage_path.is_dir() {
                print!("The path is a directory: {document_path}");
                return (Status::BadRequest, (ContentType::Text, "The specified path is a directory! Unable to proceed.\n"));
            } 

            if storage_path.is_file() {
                // TODO: implement file creation w/versioning...
                let x = storage_path.display();
                println!("The path is to a file: {x}");
                return (Status::Ok, (ContentType::Text, "File deleted!\n"))
            }

            return (Status::BadRequest, (ContentType::Text, "The specified file does not exist.\n"));
        },
        None => {
            // REVIEW: Looks to be unreachable code?
            // cURLing routes `/storage` & `/storage/` & `/storage///` yield a Some value (of empty string)
            println!("path is error"); // TODO: logger.error this
            return (Status::InternalServerError, (ContentType::Text, "Error Occurred reading path."));
        },
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/storage", routes![head, get, put, delete])
}