use actix_multipart::form::{MultipartForm, json::Json as MpJson, tempfile::TempFile};
use actix_web::{
    App, HttpResponse, HttpServer, Responder, Result,
    http::{Error, StatusCode, header::ContentType},
    post,
};
use serde::Deserialize;
use std::fs;

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
// pub async fn post_video(MultipartForm(form): MultipartForm<UploadForm>) -> impl Responder {
pub async fn post_video(form: MultipartForm<UploadForm>) -> HttpResponse {
    println!("{}", form.json.name);
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
                Ok(mut dest_file) => {
                    // We need to read the data from the file in the multipart form and write it to the destination file
                    let mut file_stream = file.as_file();

                    // Copy data from the multipart stream to the file on disk
                    match tokio::io::copy(&mut file_stream, &mut dest_file).await {
                        Ok(_) => {
                            return HttpResponse::Ok()
                                .content_type(ContentType::plaintext())
                                .body(format!(
                                    "Uploaded file {}, with size: {}",
                                    form.json.name, form.file.size
                                ));
                        }
                        Err(_) => {
                            return HttpResponse::new(StatusCode::from_u16(500).unwrap());
                        }
                    }
                }
                Err(_) => {
                    return HttpResponse::new(StatusCode::from_u16(500).unwrap());
                }
            }
        //     return HttpResponse::Ok()
        //         .content_type(ContentType::plaintext())
        //         .body(format!(
        //             "Uploaded file {}, with size: {}",
        //             form.json.name, form.file.size
        //         ));
        // }
    };
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || App::new().service(post_video))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
