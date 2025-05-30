use std::io;

use actix_files::{Files};
use actix_session::{Session, SessionMiddleware, storage::CookieSessionStore};
use actix_web::{
    App, HttpRequest, HttpResponse, HttpServer, Responder, Result, error, get,
    http::{
        Method, StatusCode,
        header::{self, ContentType},
    },
    middleware, web,
};

static SESSION_SIGNING_KEY: &[u8] = &[0; 64];

#[get("/index")]
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
        .body(include_str!("../static/index.html")))
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
            .service(Files::new("/static", "static").show_files_listing())
            // redirect to index
            .service(
                web::resource("/").route(web::get().to(|req: HttpRequest| async move {
                    println!("{req:?}");
                    HttpResponse::Found()
                        .insert_header((header::LOCATION, "static/index.html"))
                        .finish()
                })),
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
