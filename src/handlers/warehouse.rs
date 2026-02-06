use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, TextExpressionMethods, insert_into};
use crate::{AppState, helpers::{establish_connection, is_htmx}, models::{EquipmentItem, EquipmentType}};
use actix_web::{HttpRequest, HttpResponse, get, post, web};
use serde::{Deserialize, Serialize};
use tera::Context;

use crate::schema;

#[derive(Serialize, Deserialize, Debug)]
struct EquipmentTypeForm {
    type_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct EquipmentTypeQuery {
    query: Option<String>,
}

// Html

#[get("")]
async fn index(data: web::Data<AppState>) -> HttpResponse {
    let page = data.tera.render("pages/warehouse.html",
        &tera::Context::new())
        .unwrap();

    return HttpResponse::Ok()
        .content_type("text/html")
        .body(page);
}

#[get("/equipment-type/new")]
async fn equipment_type_new_form(
    request: HttpRequest,
    data: web::Data<AppState>
) -> HttpResponse {
    if !is_htmx(&request) {
        return HttpResponse::NotFound().finish();
    }

    let page = data.tera.render("partials/warehouse-type-new.html",
        &tera::Context::new())
        .unwrap();

    return HttpResponse::Ok()
        .content_type("text/html")
        .body(page);
}

// Form

#[post("/equipment-type/new")]
async fn create_equipment_type(
    form: web::Form<EquipmentTypeForm>,
    data: web::Data<AppState>
) -> HttpResponse {
    let result = web::block(move || {
        use schema::equipment_type::dsl::*;

        let mut connection = establish_connection();

        insert_into(equipment_type)
            .values(name.eq(&form.type_name))
            .execute(&mut connection)

    }).await.unwrap();

    let mut context = Context::new();
    let page = match result {
        Ok(_) => {
            context.insert("success", "Insert operation succeeded.");
            data.tera.render("partials/success.html", &context).unwrap()
        }
        Err(_) => {
            context.insert("error", "Insert operation failed.");
            data.tera.render("partials/error.html", &context).unwrap()
        }
    };

    match result {
        Ok(_) => HttpResponse::Ok()
            .append_header(("HX-Trigger", "equipmentTypeCreated"))
            .content_type("text/html")
            .body(page),

        Err(_) => HttpResponse::UnprocessableEntity()
            .content_type("text/html")
            .body(page),
    }
}

#[get("/equipment-types")]
async fn query_equipment_types(
    query: web::Query<EquipmentTypeQuery>,
    data: web::Data<AppState>,
) -> HttpResponse {
    let items = web::block(move || {
        use schema::equipment_type::dsl::*;
        let mut connection = establish_connection();
        let query = &query.query.clone().unwrap_or_default();

        if !query.is_empty() {
            equipment_type
                .filter(name.like(format!("%{}%", &query)))
                .load::<EquipmentType>(&mut connection)
                .unwrap()
        } else {
            equipment_type
                .load::<EquipmentType>(&mut connection)
                .unwrap()
        }
    })
    .await
    .unwrap();

    let mut context = Context::new();
    context.insert("items", &items);
    let page = data.tera.render("partials/warehouse-type-list.html", &context)
        .unwrap();

    HttpResponse::Ok()
        .content_type("text/html")
        .body(page)
}

#[get("/equipment-types/{id}")]
async fn get_equipment_type(
    path: web::Path<i32>,
    data: web::Data<AppState>,
) -> HttpResponse {
    let equipment_type_id = path.into_inner();

    let info = web::block(move || {
        use schema::equipment_type::dsl::*;
        let mut connection = establish_connection();

        equipment_type
            .filter(id.eq(equipment_type_id))
            .first::<EquipmentType>(&mut connection)
            .unwrap()
    })
    .await
    .unwrap();

    let items = web::block(move || {
        use schema::equipment_item::dsl::*;
        let mut connection = establish_connection();

        equipment_item
            .filter(type_id.eq(equipment_type_id))
            .load::<EquipmentItem>(&mut connection)
            .unwrap()
    })
    .await
    .unwrap();

    let mut context = Context::new();
    context.insert("serial", &info.name);
    context.insert("items", &items);
    let page = data.tera.render("partials/warehouse-type-select.html", &context)
        .unwrap();

    HttpResponse::Ok()
        .content_type("text/html")
        .body(page)
}
