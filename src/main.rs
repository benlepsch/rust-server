use std::io;

use actix_session::{Session, SessionMiddleware, storage::CookieSessionStore};
use actix_web::{
    App, Either, HttpRequest, HttpResponse, HttpServer, Responder, Result, get,
    http::{
        Method, StatusCode,
        header::{self, ContentType},
    },
    web,
};
use actix_files::Files;

static SESSION_SIGNING_KEY: &[u8] = &[0; 64];

#[get("/")]
async fn index(req: HttpRequest, session: Session) -> Result<HttpResponse> {
    println!("{req:?}");

    let mut counter = 1;
    if let Some(count) = session.get::<i32>("counter")? {
        println!("SESSION value: {count}");
        counter = count + 1;
    }

    session.insert("counter", counter)?;

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::html())
        .body(include_str!("../build/index.html")))
}

async fn default_handler(req_method: Method) -> Result<impl Responder> {
    match req_method {
        Method::GET => {
            Ok(Either::Left(HttpResponse::build(StatusCode::OK)
                .content_type(ContentType::html())
                .body(include_str!("../build/index.html"))))
        }
        _ => Ok(Either::Right(HttpResponse::MethodNotAllowed().finish())),
    }
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    let key = actix_web::cookie::Key::from(SESSION_SIGNING_KEY);
    
    HttpServer::new(move || {
        App::new()
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), key.clone())
                    .cookie_secure(false)
                    .build(),
            )
            .service(index)
            .service(Files::new("/", "build"))
            .default_service(web::to(default_handler))        
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
