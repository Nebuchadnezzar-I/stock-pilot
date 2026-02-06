use actix_web::{HttpResponse, get, http::header};

#[get("/")]
async fn index() -> HttpResponse {
    return HttpResponse::SeeOther()
        .append_header((header::LOCATION, "/warehouse"))
        .finish()
}
