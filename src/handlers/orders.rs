use actix_web::{HttpResponse, get, web};
use crate::AppState;

static ORDERS_TEMPLATE: &'static str = "pages/orders.html";

#[get("/orders")]
async fn index(data: web::Data<AppState>) -> HttpResponse {
    let page = data.tera.render(ORDERS_TEMPLATE,
        &tera::Context::new())
        .unwrap();

    return HttpResponse::Ok()
        .content_type("text/html")
        .body(page);
}
