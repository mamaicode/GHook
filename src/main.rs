use std::{env, arch::x86_64::_mm256_packus_epi16};

use ntex::web::{self, HttpResponse, HttpRequest};

use hmac_sha256::HMAC;

#[web::post("/webhook")]
async fn webhook(
  req: HttpRequest,
  body: web::types::Json<serde_json::Value>,
) -> HttpResponse {
  let secret_token = env::var("SECRET_TOKEN)").unwrap();
  // .map_err(|_| HttpResponse::InternalServerError().finish())?;
  let signature_header = req.headers().get("X-Hub-Signature-256").unwrap();
  // .ok_or(HttpResponse::InternalServerError().finish())?;

  let payload_bytes = serde_json::to_string(&body.into_inner()).unwrap();
  // .map_err(|_| HttpResponse::InternalServerError().finish())?;

  let bitchleo = HMAC::mac(payload_bytes, secret_token);

  let hex_string = bitchleo
    .into_iter()
    .map(|b| format!("{:02x}", b))
    .collect::<String>();

  println!("{}", hex_string);
  println!("{:#?}", signature_header);

  HttpResponse::Ok().finish()
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
  web::server(|| web::App::new().service(webhook))
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
