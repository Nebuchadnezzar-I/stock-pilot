pub mod warehouse {
    pub mod routes {
        use crate::handlers::prelude::*;

        pub async fn index(tera: web::Data<Tera>) -> HttpResponse {
            let page = tera.render("pages/warehouse.html",
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
            let found_types = web::block(move || {
                use crate::database::schema::equipment_type::dsl::*;
                let query = &query.query.clone().unwrap_or_default();

                let mut connection = pool.get()
                    .expect("Could not get connection from pool!");

                if query.is_empty() {
                return equipment_type
                    .load::<EquipmentType>(&mut connection)
                    .unwrap();
                }

                return equipment_type
                    .filter(name.like(format!("%{}%", query)))
                    .load::<EquipmentType>(&mut connection)
                    .unwrap();

            }).await.unwrap();

            let mut context = Context::new();
            context.insert("types", &found_types);

            let page = tera.render(
                "fragments/warehouse/type-list.html",
                &context).unwrap();

            return HttpResponse::Ok()
                .content_type("text/html")
                .body(page);
        }

        pub async fn type_new(
            tera: web::Data<Tera>
        ) -> HttpResponse {
            let page = tera.render(
                "fragments/warehouse/type-new.html",
                &Context::new()).unwrap();

            return HttpResponse::Ok()
                .content_type("text/html")
                .body(page);
        }

        pub async fn item_new(
            type_id: web::Path<i32>,
            tera: web::Data<Tera>
        ) -> HttpResponse {
            let mut context = Context::new();
            context.insert("type_id", &type_id.clone());

            let page = tera.render(
                "fragments/warehouse/item-new.html",
                &context).unwrap();

            return HttpResponse::Ok()
                .content_type("text/html")
                .body(page);
        }

        pub async fn item_update(
            item_id: web::Path<i32>,
            pool: web::Data<DbPool>,
            tera: web::Data<Tera>
        ) -> HttpResponse {
            let item_info = web::block(move || {
                use crate::database::schema::equipment_item::dsl::*;

                let mut connection = pool.get()
                    .expect("Could not get connection from pool!");

                return equipment_item
                    .filter(id.eq(item_id.clone()))
                    .first::<EquipmentItem>(&mut connection)
                    .unwrap();

            }).await.unwrap();

            let mut context = Context::new();
            context.insert("item", &item_info);

            let page = tera.render(
                "fragments/warehouse/item-update.html",
                &context).unwrap();

            return HttpResponse::Ok()
                .content_type("text/html")
                .body(page);
        }

        pub async fn type_delete(
            type_id: web::Path<i32>,
            pool: web::Data<DbPool>,
            tera: web::Data<Tera>
        ) -> HttpResponse {
            let type_info = web::block(move || {
                use crate::database::schema::equipment_type::dsl::*;

                let mut connection = pool.get()
                    .expect("Could not get connection from pool!");

                return equipment_type
                    .filter(id.eq(type_id.clone()))
                    .first::<EquipmentType>(&mut connection)
                    .unwrap();

            }).await.unwrap();

            let mut context = Context::new();
            context.insert("type", &type_info);

            let page = tera.render(
                "fragments/warehouse/type-delete.html",
                &context).unwrap();

            return HttpResponse::Ok()
                .content_type("text/html")
                .body(page);
        }

        pub async fn type_select(
            type_id: web::Path<i32>,
            pool: web::Data<DbPool>,
            tera: web::Data<Tera>
        ) -> HttpResponse {
            let type_info = web::block(move || {
                use crate::database::schema::equipment_type::dsl::*;

                let mut connection = pool.get()
                    .expect("Could not get connection from pool!");

                return equipment_type
                    .filter(id.eq(type_id.clone()))
                    .first::<EquipmentType>(&mut connection)
                    .unwrap();

            }).await.unwrap();

            let mut context = Context::new();
            context.insert("type", &type_info);

            let page = tera.render(
                "fragments/warehouse/type-select.html",
                &context).unwrap();

            return HttpResponse::Ok()
                .content_type("text/html")
                .body(page);
        }

        pub async fn type_update(
            type_id: web::Path<i32>,
            pool: web::Data<DbPool>,
            tera: web::Data<Tera>
        ) -> HttpResponse {
            let type_info = web::block(move || {
                use crate::database::schema::equipment_type::dsl::*;

                let mut connection = pool.get()
                    .expect("Could not get connection from pool!");

                return equipment_type
                    .filter(id.eq(type_id.clone()))
                    .first::<EquipmentType>(&mut connection)
                    .unwrap();

            }).await.unwrap();

            let mut context = Context::new();
            context.insert("type", &type_info);

            let page = tera.render(
                "fragments/warehouse/type-update.html",
                &context).unwrap();

            return HttpResponse::Ok()
                .content_type("text/html")
                .body(page);
        }

        pub async fn item_delete(
            item_id: web::Path<i32>,
            pool: web::Data<DbPool>,
            tera: web::Data<Tera>
        ) -> HttpResponse {
            let item_info = web::block(move || {
                use crate::database::schema::equipment_item::dsl::*;

                let mut connection = pool.get()
                    .expect("Could not get connection from pool!");

                return equipment_item
                    .filter(id.eq(item_id.clone()))
                    .first::<EquipmentItem>(&mut connection)
                    .unwrap();

            }).await.unwrap();

            let mut context = Context::new();
            context.insert("item", &item_info);

            let page = tera.render(
                "fragments/warehouse/item-delete.html",
                &context).unwrap();

            return HttpResponse::Ok()
                .content_type("text/html")
                .body(page);
        }

        pub async fn select_item_for_type(
            type_id: web::Path<i32>,
            pool: web::Data<DbPool>,
            tera: web::Data<Tera>
        ) -> HttpResponse {
            let arg_type_id = type_id.clone();

            let items = web::block(move || {
                use crate::database::schema::equipment_item::dsl::*;

                let mut connection = pool.get()
                    .expect("Could not get connection from pool!");

                return equipment_item
                    .filter(type_id.eq(arg_type_id.clone()))
                    .load::<EquipmentItem>(&mut connection)
                    .unwrap();

            }).await.unwrap();

            let mut context = Context::new();
            context.insert("items", &items);

            let page = tera.render(
                "fragments/warehouse/item-list.html",
                &context).unwrap();

            return HttpResponse::Ok()
                .content_type("text/html")
                .body(page);
        }
    }

    pub mod store {
        use crate::handlers::prelude::*;
        use crate::database::database::DbPool;
        use diesel::{delete, insert_into};
        use diesel::dsl::update;

        #[derive(Deserialize, Serialize)]
        pub struct TypeCreateBody {
            name: String
        }

        pub async fn type_create(
            body: web::Form<TypeCreateBody>,
            pool: web::Data<DbPool>,
            tera: web::Data<Tera>
        ) -> HttpResponse {
            let result = web::block(move || {
                use crate::database::schema::equipment_type::dsl::*;

                let mut connection = pool.get()
                    .expect("Could not get connection from pool!");

                return insert_into(equipment_type)
                    .values(name.eq(&body.name))
                    .execute(&mut connection);

            }).await.unwrap();

            let mut context = Context::new();
            let page = match result {
                Ok(_) => {
                    context.insert("success", "Type was created.");
                    tera.render("fragments/success.html", &context).unwrap()
                }

                Err(_) => {
                    context.insert("error", "Type could not be created.");
                    tera.render("fragments/error.html", &context).unwrap()
                }
            };

            match result {
                Ok(_) => HttpResponse::Ok()
                    .append_header(("HX-Trigger", "equipmentTypesChanged"))
                    .content_type("text/html")
                    .body(page),

                Err(_) => HttpResponse::UnprocessableEntity()
                    .content_type("text/html")
                    .body(page),
            }
        }

        #[derive(Deserialize, Serialize)]
        pub struct TypeDeleteBody {
            id: i32
        }

        pub async fn type_delete(
            body: web::Form<TypeDeleteBody>,
            pool: web::Data<DbPool>,
            tera: web::Data<Tera>
        ) -> HttpResponse {
            let result = web::block(move || {
                use crate::database::schema::equipment_type::dsl::*;

                let mut connection = pool.get()
                    .expect("Could not get connection from pool!");

                return delete(equipment_type)
                    .filter(id.eq(&body.id))
                    .execute(&mut connection);

            }).await.unwrap();

            let mut context = Context::new();
            let page = match result {
                Ok(_) => {
                    context.insert("success", "Type delete operation succeeded.");
                    tera.render("fragments/success.html", &context).unwrap()
                }

                Err(_) => {
                    context.insert("error", "Type delete operation failed.");
                    tera.render("fragments/error.html", &context).unwrap()
                }
            };

            match result {
                Ok(_) => HttpResponse::Ok()
                    .append_header(("HX-Trigger", "equipmentTypesChanged"))
                    .content_type("text/html")
                    .body(page),

                Err(_) => HttpResponse::UnprocessableEntity()
                    .content_type("text/html")
                    .body(page),
            }
        }

        #[derive(Deserialize, Serialize)]
        pub struct TypeUpdateBody {
            id: i32,
            name: String
        }

        pub async fn type_update(
            body: web::Form<TypeUpdateBody>,
            pool: web::Data<DbPool>,
            tera: web::Data<Tera>
        ) -> HttpResponse {
            let result = web::block(move || {
                use crate::database::schema::equipment_type::dsl::*;

                let mut connection = pool.get()
                    .expect("Could not get connection from pool!");

                return update(equipment_type)
                    .filter(id.eq(&body.id))
                    .set(name.eq(&body.name))
                    .execute(&mut connection);

            }).await.unwrap();

            let mut context = Context::new();
            let page = match result {
                Ok(_) => {
                    context.insert("success", "Type update operation succeeded.");
                    tera.render("fragments/success.html", &context).unwrap()
                }

                Err(_) => {
                    context.insert("error", "Type update operation failed.");
                    tera.render("fragments/error.html", &context).unwrap()
                }
            };

            match result {
                Ok(_) => HttpResponse::Ok()
                    .append_header(("HX-Trigger", "equipmentTypesChanged"))
                    .content_type("text/html")
                    .body(page),

                Err(_) => HttpResponse::UnprocessableEntity()
                    .content_type("text/html")
                    .body(page),
            }
        }

        #[derive(Deserialize, Serialize)]
        pub struct ItemCreateBody {
            type_id: i32,
            serial: String
        }

        pub async fn item_create(
            body: web::Form<ItemCreateBody>,
            pool: web::Data<DbPool>,
            tera: web::Data<Tera>
        ) -> HttpResponse {
            let result = web::block(move || {
                use crate::database::schema::equipment_item::dsl::*;

                let mut connection = pool.get()
                    .expect("Could not get connection from pool!");

                return insert_into(equipment_item)
                    .values((type_id.eq(&body.type_id),
                            serial.eq(&body.serial)))
                    .execute(&mut connection);

            }).await.unwrap();

            let mut context = Context::new();
            let page = match result {
                Ok(_) => {
                    context.insert("success", "Item was created.");
                    tera.render("fragments/success.html", &context).unwrap()
                }

                Err(_) => {
                    context.insert("error", "Item could not be created.");
                    tera.render("fragments/error.html", &context).unwrap()
                }
            };

            match result {
                Ok(_) => HttpResponse::Ok()
                    .append_header(("HX-Trigger", "equipmentItemsChanged"))
                    .content_type("text/html")
                    .body(page),

                Err(_) => HttpResponse::UnprocessableEntity()
                    .content_type("text/html")
                    .body(page),
            }
        }

        #[derive(Deserialize, Serialize)]
        pub struct ItemUpdateBody {
            id: i32,
            serial: String,
        }

        pub async fn item_update(
            body: web::Form<ItemUpdateBody>,
            pool: web::Data<DbPool>,
            tera: web::Data<Tera>
        ) -> HttpResponse {
            let result = web::block(move || {
                use crate::database::schema::equipment_item::dsl::*;

                let mut connection = pool.get()
                    .expect("Could not get connection from pool!");

                return update(equipment_item)
                    .filter(id.eq(&body.id))
                    .set(serial.eq(&body.serial))
                    .execute(&mut connection);

            }).await.unwrap();

            let mut context = Context::new();
            let page = match result {
                Ok(_) => {
                    context.insert("success", "Item was updated.");
                    tera.render("fragments/success.html", &context).unwrap()
                }

                Err(_) => {
                    context.insert("error", "Item could not be updated.");
                    tera.render("fragments/error.html", &context).unwrap()
                }
            };

            match result {
                Ok(_) => HttpResponse::Ok()
                    .append_header(("HX-Trigger", "equipmentItemsChanged"))
                    .content_type("text/html")
                    .body(page),

                Err(_) => HttpResponse::UnprocessableEntity()
                    .content_type("text/html")
                    .body(page),
            }
        }

        #[derive(Deserialize, Serialize)]
        pub struct ItemDeleteBody {
            id: i32,
        }

        pub async fn item_delete(
            body: web::Form<ItemDeleteBody>,
            pool: web::Data<DbPool>,
            tera: web::Data<Tera>
        ) -> HttpResponse {
            let result = web::block(move || {
                use crate::database::schema::equipment_item::dsl::*;

                let mut connection = pool.get()
                    .expect("Could not get connection from pool!");

                return delete(equipment_item)
                    .filter(id.eq(&body.id))
                    .execute(&mut connection);

            }).await.unwrap();

            let mut context = Context::new();
            let page = match result {
                Ok(_) => {
                    context.insert("success", "Item was deleted.");
                    tera.render("fragments/success.html", &context).unwrap()
                }

                Err(_) => {
                    context.insert("error", "Item could not be deleted.");
                    tera.render("fragments/error.html", &context).unwrap()
                }
            };

            match result {
                Ok(_) => HttpResponse::Ok()
                    .append_header(("HX-Trigger", "equipmentItemsChanged"))
                    .content_type("text/html")
                    .body(page),

                Err(_) => HttpResponse::UnprocessableEntity()
                    .content_type("text/html")
                    .body(page),
            }
        }
    }
}
