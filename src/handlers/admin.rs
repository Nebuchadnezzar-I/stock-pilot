use actix_web::{HttpResponse, get, web};
use crate::AppState;

static ADMIN_TEMPLATE: &'static str = "pages/admin.html";

#[get("/admin")]
async fn index(data: web::Data<AppState>) -> HttpResponse {
    let page = data.tera.render(ADMIN_TEMPLATE,
        &tera::Context::new())
        .unwrap();

    return HttpResponse::Ok()
        .content_type("text/html")
        .body(page);
}
