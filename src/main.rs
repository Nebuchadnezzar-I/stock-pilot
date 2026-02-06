use actix_web::{App, HttpServer, web};
use actix_files::Files;
use tera::{Tera};

mod handlers;
mod schema;
mod models;
mod helpers;

pub struct AppState {
    tera: Tera,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let tera = Tera::new("templates/**/*.html")
        .expect("Count not setup tera!");

    let state = web::Data::new(AppState {
        tera
    });

    fn static_files() -> Files {
        Files::new("/static", "./templates/static")
            .prefer_utf8(true)
            .use_last_modified(true)
            .use_etag(true)
    }

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(web::scope("/warehouse")
                .service(handlers::warehouse::index)
                .service(handlers::warehouse::equipment_type_new_form)
                .service(handlers::warehouse::get_equipment_type)
                .service(handlers::warehouse::create_equipment_type)
                .service(handlers::warehouse::query_equipment_types))

            .service(handlers::orders::index)
            .service(handlers::admin::index)
            .service(handlers::other::index)
            .service(static_files())
    })
    .bind(("127.0.0.1", 6969))?
        .run()
        .await
}
