use actix_multipart::form::{MultipartForm, json::Json as MpJson, tempfile::TempFile};
use actix_web::{
    App, HttpResponse, HttpServer,
    http::{StatusCode, header::ContentType},
    post,
};
use serde::Deserialize;
use std::fs;
use tokio::fs::File;

#[derive(Debug, Deserialize)]
struct Metadata {
    name: String,
}

#[derive(Debug, MultipartForm)]
struct UploadForm {
    #[multipart(limit = "100MB")]
    file: TempFile,
    json: MpJson<Metadata>,
}

#[post("/videos")]
pub async fn post_video(form: MultipartForm<UploadForm>) -> HttpResponse {
    let mut dir_builder: fs::DirBuilder = fs::DirBuilder::new();
    dir_builder.recursive(true);
    match dir_builder.create("uploads") {
        Err(_) => {
            return HttpResponse::new(StatusCode::from_u16(500).unwrap());
        }
        Ok(_) => {
            let file = &form.file.file;
            let file_name = form.file.file_name.as_ref().unwrap();
            let file_path = format!("uploads/{}", file_name);
            match File::create(&file_path).await {
                Err(_) => {
                    return HttpResponse::new(StatusCode::from_u16(500).unwrap());
                }
                Ok(mut dest_file) => {
                    let mut file_stream =
                        tokio::fs::File::from_std(file.as_file().try_clone().unwrap());

                    match tokio::io::copy(&mut file_stream, &mut dest_file).await {
                        Err(_) => {
                            return HttpResponse::new(StatusCode::from_u16(500).unwrap());
                        }
                        Ok(_) => {
                            return HttpResponse::Ok()
                                .content_type(ContentType::plaintext())
                                .body(format!(
                                    "Uploaded file {}, with size {}",
                                    form.json.name, form.file.size
                                ));
                        }
                    }
                }
            }
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || App::new().service(post_video))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
