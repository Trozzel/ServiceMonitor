use std::path::{Path, PathBuf};

use rocket::fs::NamedFile;
use rocket::get;

#[get("/home")]
pub async fn serve_home_page() -> Result<NamedFile, std::io::Error> {
    NamedFile::open("static/index.html").await
}

#[get("/")]
pub async fn serve_index_page() -> Result<NamedFile, std::io::Error> {
    NamedFile::open("frontend/build/index.html").await
}

#[get("/<file..>")]
pub async fn serve_files(file: PathBuf) -> Result<NamedFile, std::io::Error> {
    NamedFile::open(Path::new("frontend/build").join(file)).await
}

