pub mod admin {
    pub mod routes {
        use crate::handlers::prelude::*;
        use tera::Tera;

        pub async fn index(tera: web::Data<Tera>) -> HttpResponse {
            let page = tera.render("pages/admin.html",
                &Context::new()).unwrap();

            return HttpResponse::Ok()
                .content_type("text/html")
                .body(page);
        }
    }

    pub mod fragments {
        // use crate::handlers::prelude::*;
    }

    pub mod actions {
        // use crate::handlers::prelude::*;
    }
}
