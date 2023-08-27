use actix_files::NamedFile;
use actix_web::{HttpRequest, Result};
use std::path::PathBuf;

pub async fn index(req: HttpRequest) -> Result<NamedFile> {
    let mut path: PathBuf = PathBuf::new();
    path.push("public");
    path.push(
        req.match_info()
            .query("filename")
            .parse::<String>()
            .unwrap(),
    );
    Ok(NamedFile::open(path)?)
}
