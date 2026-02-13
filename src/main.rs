use actix_web::{App, HttpServer, guard, web};
use actix_files::Files;
use tera::{Tera};

use handlers::warehouse::warehouse;
use handlers::orders::orders;
use handlers::admin::admin;
use database::database::*;

mod database;
mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(Tera::new("templates/**/*.html").unwrap()))
            .app_data(web::Data::new(create_pool()))

            .service(web::redirect("/", "orders"))
            .service(Files::new("/static", "./templates/static")
                .prefer_utf8(true)
                .use_last_modified(true)
                .use_etag(true))

            .service(
                web::scope("/orders")
                    .route("", web::get().to(orders::routes::index))

                    .service(web::scope("/fragment")
                        .guard(guard::Header("HX-Request", "true"))
                        .route("order-search",
                            web::get().to(orders::fragments::search))
                        .route("order-new",
                            web::get().to(orders::fragments::order_new))
                        .route("order-delete/{order_id}",
                            web::get().to(orders::fragments::order_delete))
                        .route("order-select/{order_id}",
                            web::get().to(orders::fragments::order_select))
                        .route("order-update/{order_id}",
                            web::get().to(orders::fragments::order_update))

                        .route("order-stage/{order_id}",
                            web::get().to(orders::fragments::order_stage))
                        .route("order-stage/order/item-new",
                            web::get().to(orders::fragments::stage_order_item_new))
                    )

                    .service(web::scope("/store")
                        .route("order-create",
                            web::post().to(orders::store::order_create))
                        .route("order-delete",
                            web::post().to(orders::store::order_delete))
                        .route("order-update",
                            web::post().to(orders::store::order_update))
                    )
            )

            .service(
                web::scope("/warehouse")
                    .route("", web::get().to(warehouse::routes::index))

                    .service(web::scope("/fragment")
                        .guard(guard::Header("HX-Request", "true"))
                        .route("type-search",
                            web::get().to(warehouse::fragments::search))
                        .route("type-new",
                            web::get().to(warehouse::fragments::type_new))
                        .route("type-delete/{type_id}",
                            web::get().to(warehouse::fragments::type_delete))
                        .route("type-select/{type_id}",
                            web::get().to(warehouse::fragments::type_select))
                        .route("type-update/{type_id}",
                            web::get().to(warehouse::fragments::type_update))

                        .route("items-for-type/{type_id}",
                            web::get().to(warehouse::fragments::select_item_for_type))
                        .route("item-new/{type_id}",
                            web::get().to(warehouse::fragments::item_new))
                        .route("item-update/{item_id}",
                            web::get().to(warehouse::fragments::item_update))
                        .route("item-delete/{item_id}",
                            web::get().to(warehouse::fragments::item_delete))
                    )

                   .service(web::scope("/store")
                        .route("type-create",
                            web::post().to(warehouse::store::type_create))
                        .route("type-delete",
                            web::post().to(warehouse::store::type_delete))
                        .route("type-update",
                            web::post().to(warehouse::store::type_update))

                        .route("item-create",
                            web::post().to(warehouse::store::item_create))
                        .route("item-update",
                            web::post().to(warehouse::store::item_update))
                        .route("item-delete",
                            web::post().to(warehouse::store::item_delete))
                    )
            )

            .service(
                web::scope("/admin")
                    .route("", web::get().to(admin::routes::index))
            )
    })
    .bind(("127.0.0.1", 6969))?
    .run()
    .await
}
