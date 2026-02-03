use actix_web::{App, HttpServer, web};
use actix_files::Files;
use tera::{Tera};

mod templates;
mod handlers;

pub struct AppState {
    tera: Tera,
}

fn static_files() -> Files {
    Files::new("/static", "./templates/static")
        .prefer_utf8(true)
        .use_last_modified(true)
        .use_etag(true)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let tera = Tera::new("templates/**/*.html")
        .expect("Count not setup tera!");

    let state = web::Data::new(AppState {
        tera
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(handlers::home_handler)
            .service(handlers::login_handler)
            .service(static_files())
    })
    .bind(("127.0.0.1", 6969))?
    .run()
    .await
}
