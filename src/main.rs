use actix_web::{App, HttpServer, web};
use actix_files::Files;
use tera::{Tera};

mod handlers;

static ORDERS_TEMPLATE: &'static str =
    "pages/orders.html";
static WAREHOUSE_TEMPLATE: &'static str =
    "pages/warehouse.html";
static ADMIN_TEMPLATE: &'static str =
    "pages/admin.html";

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
            .service(handlers::index_handler)
            .service(handlers::orders_handler)
            .service(handlers::warehouse_handler)
            .service(handlers::admin_handler)
            .service(static_files())
    })
    .bind(("127.0.0.1", 6969))?
        .run()
        .await
}
