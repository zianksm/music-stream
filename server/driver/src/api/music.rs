use actix_web::{
    web::{Bytes, BytesMut},
    HttpResponse, HttpResponseBuilder, Responder,
};
use tokio::io::AsyncReadExt;

pub async fn stream() -> impl Responder {
    let path = std::env::current_dir().unwrap().join("music").join("yume no tsuzuki.mp3");
    println!("{}", path.display().to_string());
    let file = tokio::fs::File::open(path)
        .await
        .unwrap();

    let metadata = file.metadata().await.unwrap();
    let content_len = metadata.len() as usize;

    let mut buffer = BytesMut::with_capacity(content_len);

    let mut reader = tokio::io::BufReader::new(file);

    let _ = reader.read_buf(&mut buffer).await.unwrap();

    let contents = futures::stream::once(async move { Ok::<_, actix_web::Error>(buffer.freeze()) });

    HttpResponse::Ok()
        .content_type("audio/mpeg")
        .streaming(contents)
}
