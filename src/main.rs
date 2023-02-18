use ntex::web::{self, HttpResponse, HttpRequest};

#[web::post("/webhook")]
async fn webhook(
  req: HttpRequest,
  body: web::types::Json<serde_json::Value>,
) -> HttpResponse {
  println!("Hello req:  {req:#?}");
  println!("Hello body:  {body:#?}");
  HttpResponse::Ok().finish()
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
  web::server(|| web::App::new().service(webhook))
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
