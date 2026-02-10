pub mod orders {
    pub mod routes {
        use crate::handlers::prelude::*;
        use tera::Tera;

        pub async fn index(tera: web::Data<Tera>) -> HttpResponse {
            let page = tera.render("pages/orders.html",
                &Context::new()).unwrap();

            return HttpResponse::Ok()
                .content_type("text/html")
                .body(page);
        }
    }

    pub mod fragments {
        use crate::handlers::prelude::*;
        use crate::database::models::*;
        use crate::database::database::DbPool;

        #[derive(Deserialize, Serialize)]
        pub struct SearchQuery {
            query: Option<String>,
        }

        pub async fn search(
            query: web::Query<SearchQuery>,
            pool: web::Data<DbPool>,
            tera: web::Data<Tera>
        ) -> HttpResponse {
            let found_orders = web::block(move || {
                use crate::database::schema::orders::dsl::*;
                let query = &query.query.clone().unwrap_or_default();

                let mut connection = pool.get()
                    .expect("Could not get connection from pool!");

                if query.is_empty() {
                return orders
                    .load::<Order>(&mut connection)
                    .unwrap();
                }

                return orders
                    .filter(tag.like(format!("%{}%", query)))
                    .load::<Order>(&mut connection)
                    .unwrap();

            }).await.unwrap();

            let mut context = Context::new();
            context.insert("orders", &found_orders);

            let page = tera.render(
                "fragments/orders/order-list.html",
                &context).unwrap();

            return HttpResponse::Ok()
                .content_type("text/html")
                .body(page);
        }

        pub async fn order_new(
            tera: web::Data<Tera>
        ) -> HttpResponse {
            let page = tera.render(
                "fragments/orders/order-new.html",
                &Context::new()).unwrap();

            return HttpResponse::Ok()
                .content_type("text/html")
                .body(page);
        }

        pub async fn order_delete(
            order_id: web::Path<i32>,
            pool: web::Data<DbPool>,
            tera: web::Data<Tera>
        ) -> HttpResponse {
            let order = web::block(move || {
                use crate::database::schema::orders::dsl::*;

                let mut connection = pool.get()
                    .expect("Could not get connection from pool!");

                return orders
                    .filter(id.eq(order_id.clone()))
                    .first::<Order>(&mut connection)
                    .unwrap();

            }).await.unwrap();

            let mut context = Context::new();
            context.insert("order", &order);

            let page = tera.render(
                "fragments/orders/order-delete.html",
                &context).unwrap();

            return HttpResponse::Ok()
                .content_type("text/html")
                .body(page);
        }

        pub async fn order_select(
            order_id: web::Path<i32>,
            pool: web::Data<DbPool>,
            tera: web::Data<Tera>
        ) -> HttpResponse {
            let order_info = web::block(move || {
                use crate::database::schema::orders::dsl::*;

                let mut connection = pool.get()
                    .expect("Could not get connection from pool!");

                return orders
                    .filter(id.eq(order_id.clone()))
                    .first::<Order>(&mut connection)
                    .unwrap();

            }).await.unwrap();

            let mut context = Context::new();
            context.insert("order", &order_info);

            let page = tera.render(
                "fragments/orders/order-select.html",
                &context).unwrap();

            return HttpResponse::Ok()
                .content_type("text/html")
                .body(page);
        }

        pub async fn order_update(
            order_id: web::Path<i32>,
            pool: web::Data<DbPool>,
            tera: web::Data<Tera>
        ) -> HttpResponse {
            let order_info = web::block(move || {
                use crate::database::schema::orders::dsl::*;

                let mut connection = pool.get()
                    .expect("Could not get connection from pool!");

                return orders
                    .filter(id.eq(order_id.clone()))
                    .first::<Order>(&mut connection)
                    .unwrap();

            }).await.unwrap();

            let mut context = Context::new();
            context.insert("order", &order_info);

            let page = tera.render(
                "fragments/orders/order-update.html",
                &context).unwrap();

            return HttpResponse::Ok()
                .content_type("text/html")
                .body(page);
        }

        #[derive(Deserialize, Serialize)]
        pub struct StageQuery {
            stage: Option<String>,
        }

        pub async fn order_stage(
            _order_id: web::Path<i32>,
            query: web::Query<StageQuery>,
            tera: web::Data<Tera>
        ) -> HttpResponse {
            let mut context = Context::new();
            let stage = query.stage.as_deref().unwrap_or("order");

            let path = match stage {
                "order" => "fragments/orders/stage-items.html",
                "loaded" => "fragments/orders/stage-loaded.html",
                "unloaded" => "fragments/orders/stage-unloaded.html",
                _ => {
                    context.insert("error", "Stage could not be matched");
                    "fragments/error.html"
                }
            };

            let page = tera.render(
                path, &context).unwrap();

            return HttpResponse::Ok()
                .content_type("text/html")
                .body(page);
        }
    }

    pub mod store {
        use crate::handlers::prelude::*;

        #[derive(Deserialize, Serialize)]
        pub struct OrderCreateBody { }

        pub async fn order_create(
            _body: web::Form<OrderCreateBody>,
            pool: web::Data<DbPool>,
            tera: web::Data<Tera>
        ) -> HttpResponse {
            let result = web::block(move || {
                use crate::database::schema::orders::dsl::*;

                let mut connection = pool.get()
                    .expect("Could not get connection from pool!");

                return insert_into(orders)
                    .default_values()
                    .execute(&mut connection);

            }).await.unwrap();

            let mut context = Context::new();
            let page = match result {
                Ok(_) => {
                    context.insert("success", "Order was created.");
                    tera.render("fragments/success.html", &context).unwrap()
                }

                Err(_) => {
                    context.insert("error", "Order creation failed.");
                    tera.render("fragments/error.html", &context).unwrap()
                }
            };

            match result {
                Ok(_) => HttpResponse::Ok()
                    .append_header(("HX-Trigger", "ordersChanged"))
                    .content_type("text/html")
                    .body(page),

                Err(_) => HttpResponse::UnprocessableEntity()
                    .content_type("text/html")
                    .body(page),
            }
        }

        #[derive(Deserialize, Serialize)]
        pub struct OrderDeleteBody {
            id: i32
        }

        pub async fn order_delete(
            body: web::Form<OrderDeleteBody>,
            pool: web::Data<DbPool>,
            tera: web::Data<Tera>
        ) -> HttpResponse {
            let result = web::block(move || {
                use crate::database::schema::orders::dsl::*;

                let mut connection = pool.get()
                    .expect("Could not get connection from pool!");

                return delete(orders)
                    .filter(id.eq(&body.id))
                    .execute(&mut connection);

            }).await.unwrap();

            let mut context = Context::new();
            let page = match result {
                Ok(_) => {
                    context.insert("success", "Order delete operation succeeded.");
                    tera.render("fragments/success.html", &context).unwrap()
                }

                Err(_) => {
                    context.insert("error", "Order delete operation failed.");
                    tera.render("fragments/error.html", &context).unwrap()
                }
            };

            match result {
                Ok(_) => HttpResponse::Ok()
                    .append_header(("HX-Trigger", "ordersChanged"))
                    .content_type("text/html")
                    .body(page),

                Err(_) => HttpResponse::UnprocessableEntity()
                    .content_type("text/html")
                    .body(page),
            }
        }

        #[derive(Deserialize, Serialize)]
        pub struct OrderUpdateBody {
            id: i32,
            tag: String
        }

        pub async fn order_update(
            body: web::Form<OrderUpdateBody>,
            pool: web::Data<DbPool>,
            tera: web::Data<Tera>
        ) -> HttpResponse {
            let result = web::block(move || {
                use crate::database::schema::orders::dsl::*;

                let mut connection = pool.get()
                    .expect("Could not get connection from pool!");

                return update(orders)
                    .filter(id.eq(&body.id))
                    .set(tag.eq(&body.tag))
                    .execute(&mut connection);

            }).await.unwrap();

            let mut context = Context::new();
            let page = match result {
                Ok(_) => {
                    context.insert("success", "Order update peration succeeded.");
                    tera.render("fragments/success.html", &context).unwrap()
                }

                Err(_) => {
                    context.insert("error", "Order update operation failed.");
                    tera.render("fragments/error.html", &context).unwrap()
                }
            };

            match result {
                Ok(_) => HttpResponse::Ok()
                    .append_header(("HX-Trigger", "ordersChanged"))
                    .content_type("text/html")
                    .body(page),

                Err(_) => HttpResponse::UnprocessableEntity()
                    .content_type("text/html")
                    .body(page),
            }
        }
    }
}
