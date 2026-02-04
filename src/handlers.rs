use actix_web::{HttpResponse, get, http::header, web};
use crate::{ADMIN_TEMPLATE, AppState, ORDERS_TEMPLATE, WAREHOUSE_TEMPLATE};

#[get("/")]
async fn index_handler() -> HttpResponse {
    return HttpResponse::SeeOther()
        .append_header((header::LOCATION, "/warehouse"))
        .finish()
}

#[get("/orders")]
async fn orders_handler(data: web::Data<AppState>) -> HttpResponse {
    let page = data.tera.render(ORDERS_TEMPLATE,
        &tera::Context::new())
        .unwrap();

    return HttpResponse::Ok()
        .content_type("text/html")
        .body(page);
}

#[get("/warehouse")]
async fn warehouse_handler(data: web::Data<AppState>) -> HttpResponse {
    let page = data.tera.render(WAREHOUSE_TEMPLATE,
        &tera::Context::new())
        .unwrap();

    return HttpResponse::Ok()
        .content_type("text/html")
        .body(page);
}

#[get("/admin")]
async fn admin_handler(data: web::Data<AppState>) -> HttpResponse {
    let page = data.tera.render(ADMIN_TEMPLATE,
        &tera::Context::new())
        .unwrap();

    return HttpResponse::Ok()
        .content_type("text/html")
        .body(page);
}
