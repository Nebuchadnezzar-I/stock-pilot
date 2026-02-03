use actix_web::{HttpResponse, get, http::header, web};
use crate::{AppState, templates::PageLogin};



#[get("/")]
async fn home_handler() -> HttpResponse {
    return HttpResponse::SeeOther()
        .append_header((header::LOCATION, "/login"))
        .finish()
}

#[get("/login")]
async fn login_handler(data: web::Data<AppState>) -> HttpResponse {
    let login_page = PageLogin { title: "Login" };

    let context = tera::Context::from_serialize(login_page).unwrap();
    let page = data.tera.render("home.html", &context).unwrap();

    return HttpResponse::Ok()
        .content_type("text/html")
        .body(page);
}
