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
            order_id: web::Path<i32>,
            query: web::Query<StageQuery>,
            tera: web::Data<Tera>
        ) -> HttpResponse {
            let mut context = Context::new();
            context.insert("order_id", &order_id.to_owned());
            let stage = query.stage.as_deref().unwrap_or("order");

            let path = match stage {
                "order" => "fragments/orders/stage-order.html",
                "loaded" => "fragments/orders/stage-loaded.html",
                "unloaded" => "fragments/orders/stage-returned.html",
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

        pub async fn order_lines(
            order_id: web::Path<i32>,
            pool: web::Data<DbPool>,
            tera: web::Data<Tera>
        ) -> HttpResponse {
            let o_id = *order_id;

            let order_lines= web::block(move || {
                use diesel::QueryDsl;

                use crate::database::schema::order_line::dsl as ol;
                use crate::database::schema::equipment_type::dsl as et;

                let mut connection = pool.get()
                    .expect("Could not get connection from pool!");

                return ol::order_line
                    .inner_join(et::equipment_type)
                    .filter(ol::order_id.eq(o_id))
                    .load::<(OrderLine, EquipmentType)>(&mut connection)
                    .unwrap();
            }).await.unwrap();

            let mut context = Context::new();
            context.insert("order_lines", &order_lines);
            context.insert("order_id", &o_id);

            let page = tera.render(
                "fragments/orders/stage-order-ol-list.html",
                &context).unwrap();

            return HttpResponse::Ok()
                .content_type("text/html")
                .body(page);
        }

        pub async fn order_line_new(
            order_id: web::Path<i32>,
            pool: web::Data<DbPool>,
            tera: web::Data<Tera>
        ) -> HttpResponse {
            let o_id = *order_id;
            let eq_types = web::block(move || {
                use crate::database::schema::equipment_type::dsl::*;

                let mut connection = pool.get()
                    .expect("Could not get connection from pool!");

                return equipment_type
                    .load::<EquipmentType>(&mut connection)
                    .unwrap();

            }).await.unwrap();

            let mut context = Context::new();
            context.insert("order_id", &o_id);
            context.insert("eq_types", &eq_types);

            let page = tera.render(
                "fragments/orders/stage-order-ol-new.html",
                &context).unwrap();

            return HttpResponse::Ok()
                .content_type("text/html")
                .body(page);
        }

        pub async fn order_line_delete(
            path: web::Path<(i32, i32)>,
            pool: web::Data<DbPool>,
            tera: web::Data<Tera>
        ) -> HttpResponse {
            let (o_id, l_id) = path.into_inner();

            let eq_type = web::block(move || {
                use crate::database::schema::order_line::dsl as ol;
                use crate::database::schema::equipment_type::dsl as et;

                let mut connection = pool.get()
                    .expect("Could not get connection from pool!");

                return ol::order_line
                    .inner_join(et::equipment_type)
                    .filter(ol::id.eq(l_id))
                    .first::<(OrderLine, EquipmentType)>(&mut connection)
                    .unwrap();

            }).await.unwrap();

            let mut context = Context::new();
            context.insert("order_id", &o_id);
            context.insert("line_id", &l_id);
            context.insert("eq_type", &eq_type);

            let page = tera.render(
                "fragments/orders/stage-order-ol-delete.html",
                &context).unwrap();

            return HttpResponse::Ok()
                .content_type("text/html")
                .body(page);
        }

        pub async fn order_line_update(
            line_id: web::Path<i32>,
            pool: web::Data<DbPool>,
            tera: web::Data<Tera>
        ) -> HttpResponse {
            let l_id = *line_id;

            let ((order_line, order_line_eq_type), eq_types) = web::block(move || {
                use crate::database::schema::order_line::dsl as ol;
                use crate::database::schema::equipment_type::dsl as et;

                let mut connection = pool
                    .get()
                    .expect("Could not get connection from pool!");

                let order_line_with_type = ol::order_line
                    .inner_join(et::equipment_type)
                    .filter(ol::id.eq(l_id))
                    .first::<(OrderLine, EquipmentType)>(&mut connection)?;

                let eq_types = et::equipment_type
                    .load::<EquipmentType>(&mut connection)?;

                Ok::<_, diesel::result::Error>((order_line_with_type, eq_types))
            }).await.unwrap().unwrap();

            let mut context = Context::new();
            context.insert("order_line", &order_line);
            context.insert("order_line_eq_type", &order_line_eq_type);
            context.insert("eq_types", &eq_types);

            let page = tera.render(
                "fragments/orders/stage-order-ol-update.html",
                &context).unwrap();

            return HttpResponse::Ok()
                .content_type("text/html")
                .body(page);
        }

        pub async fn loaded_stage(
            order_id: web::Path<i32>,
            tera: web::Data<Tera>
        ) -> HttpResponse {
            let mut context = Context::new();
            context.insert("order_id", &order_id.clone());

            let page = tera.render(
                "fragments/orders/stage-loaded.html",
                &context).unwrap();

            return HttpResponse::Ok()
                .content_type("text/html")
                .body(page);
        }

        pub async fn loaded_lines(
            order_id: web::Path<i32>,
            pool: web::Data<DbPool>,
            tera: web::Data<Tera>
        ) -> HttpResponse {
            let o_id = *order_id;

            let lines = web::block(move || {
                use diesel::prelude::*;

                use crate::database::schema::order_line::dsl as ol;
                use crate::database::schema::equipment_type::dsl as et;
                use crate::database::schema::dispatch_item::dsl as di;
                use crate::database::schema::equipment_item::dsl as ei;

                let mut connection = pool.get().expect("Could not get connection from pool!");

                return ol::order_line
                    .inner_join(et::equipment_type)
                    .left_join(
                        di::dispatch_item
                        .left_join(ei::equipment_item)
                    )
                    .select((
                            OrderLine::as_select(),
                            EquipmentType::as_select(),
                            Option::<DispatchItem>::as_select(),
                            Option::<EquipmentItem>::as_select(),
                    ))
                    .load::<(
                        OrderLine,
                        EquipmentType,
                        Option<DispatchItem>,
                        Option<EquipmentItem>,
                    )>(&mut connection)
                    .unwrap();

            }).await.unwrap();

            let mut context = Context::new();
            context.insert("order_id", &o_id);
            context.insert("lines", &lines);

            let page = tera.render(
                "fragments/orders/stage-loaded-list.html",
                &context).unwrap();

            return HttpResponse::Ok()
                .content_type("text/html")
                .body(page);
        }
    }

    pub mod store {
        use crate::handlers::prelude::*;
        use crate::database::models::*;

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

        #[derive(Deserialize, Serialize)]
        pub struct OrderLineCreateBody {
            order_id: i32,
            amount: i32,
            equipment_name: String
        }

        pub async fn order_line_create(
            body: web::Form<OrderLineCreateBody>,
            pool: web::Data<DbPool>,
            tera: web::Data<Tera>
        ) -> HttpResponse {
            let result = web::block(move || {
                use crate::database::schema::order_line::dsl as ol;
                use crate::database::schema::equipment_type::dsl as et;

                let mut connection = pool.get()
                    .expect("Could not get connection from pool!");

                let eq_id = et::equipment_type
                    .filter(et::name.eq(body.equipment_name.clone()))
                    .first::<EquipmentType>(&mut connection)?;

                return insert_into(ol::order_line)
                    .values((
                        ol::order_id.eq(body.order_id),
                        ol::equipment_type_id.eq(eq_id.id),
                        ol::equipment_count.eq(body.amount),
                    ))
                    .execute(&mut connection);

            }).await.unwrap();

            let mut context = Context::new();
            let page = match result {
                Ok(_) => {
                    context.insert("success", "Order line was created.");
                    tera.render("fragments/success.html", &context).unwrap()
                }

                Err(_) => {
                    context.insert("error", "Order line could not be created.");
                    tera.render("fragments/error.html", &context).unwrap()
                }
            };

            match result {
                Ok(_) => HttpResponse::Ok()
                    .append_header(("HX-Trigger", "orderLinesChanged"))
                    .content_type("text/html")
                    .body(page),

                Err(_) => HttpResponse::UnprocessableEntity()
                    .content_type("text/html")
                    .body(page),
            }
        }

        #[derive(Deserialize, Serialize)]
        pub struct OrderLineDeleteBody {
            line_id: i32,
        }

        pub async fn order_line_delete(
            body: web::Form<OrderLineDeleteBody>,
            pool: web::Data<DbPool>,
            tera: web::Data<Tera>
        ) -> HttpResponse {
            let result = web::block(move || {
                use crate::database::schema::order_line::dsl::*;

                let mut connection = pool.get()
                    .expect("Could not get connection from pool!");

                return delete(order_line)
                    .filter(id.eq(&body.line_id))
                    .execute(&mut connection);

            }).await.unwrap();

            let mut context = Context::new();
            let page = match result {
                Ok(_) => {
                    context.insert("success", "Order line was removed.");
                    tera.render("fragments/success.html", &context).unwrap()
                }

                Err(_) => {
                    context.insert("error", "Order line could not be removed.");
                    tera.render("fragments/error.html", &context).unwrap()
                }
            };

            match result {
                Ok(_) => HttpResponse::Ok()
                    .append_header(("HX-Trigger", "orderLinesChanged"))
                    .content_type("text/html")
                    .body(page),

                Err(_) => HttpResponse::UnprocessableEntity()
                    .content_type("text/html")
                    .body(page),
            }
        }

        #[derive(Deserialize, Serialize)]
        pub struct OrderLineUpdateBody {
            equipment_name: String,
            line_id: i32,
            amount: i32,
        }

        pub async fn order_line_update(
            body: web::Form<OrderLineUpdateBody>,
            pool: web::Data<DbPool>,
            tera: web::Data<Tera>
        ) -> HttpResponse {
            let result = web::block(move || {
                use crate::database::schema::order_line::dsl as ol;
                use crate::database::schema::equipment_type::dsl as et;

                let mut connection = pool.get()
                    .expect("Could not get connection from pool!");

                let eq_type = et::equipment_type
                    .filter(et::name.eq(body.equipment_name.clone()))
                    .first::<EquipmentType>(&mut connection)?;

                return update(ol::order_line)
                    .filter(ol::id.eq(&body.line_id))
                    .set((
                        ol::equipment_count.eq(body.amount),
                        ol::equipment_type_id.eq(eq_type.id),
                    ))
                    .execute(&mut connection);

            }).await.unwrap();

            let mut context = Context::new();
            let page = match result {
                Ok(_) => {
                    context.insert("success", "Order line was updated.");
                    tera.render("fragments/success.html", &context).unwrap()
                }

                Err(_) => {
                    context.insert("error", "Order line could not be updateed.");
                    tera.render("fragments/error.html", &context).unwrap()
                }
            };

            match result {
                Ok(_) => HttpResponse::Ok()
                    .append_header(("HX-Trigger", "orderLinesChanged"))
                    .content_type("text/html")
                    .body(page),

                Err(_) => HttpResponse::UnprocessableEntity()
                    .content_type("text/html")
                    .body(page),
            }
        }
    }
}
