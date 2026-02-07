use crate::{AppState, helpers::establish_connection, models::{EquipmentItem, EquipmentType}};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, TextExpressionMethods, dsl::{delete, update}, insert_into};
use actix_web::{HttpResponse, get, post, web};
use serde::{Deserialize, Serialize};
use tera::Context;

use crate::schema;

//
// Page
//

#[get("")]
async fn index(app: web::Data<AppState>) -> HttpResponse {
    let context = Context::new();
    let page = app.tera.render("pages/warehouse.html",
        &context).unwrap();

    return HttpResponse::Ok()
        .content_type("text/html")
        .body(page);
}

//
// Page
//

//
// Fragments
//

// Items

#[get("/equipment-item/remove/{item_id}")]
async fn form_remove_equipment_item(
    item_id: web::Path<i32>,
    data: web::Data<AppState>
) -> HttpResponse {
    let mut context = Context::new();
    context.insert("item_id", &item_id.to_owned());
    let page = data.tera.render(
        "partials/warehouse/item-delete.html", 
        &context).unwrap();

    return HttpResponse::Ok()
        .content_type("text/html")
        .body(page);
}

#[get("/equipment-item/new/{type_id}")]
async fn form_new_equipment_item(
    type_id: web::Path<i32>,
    data: web::Data<AppState>
) -> HttpResponse {
    let mut context = Context::new();
    context.insert("type_id", &type_id.to_owned());
    let page = data.tera.render(
        "partials/warehouse/item-new.html", 
        &context).unwrap();

    return HttpResponse::Ok()
        .content_type("text/html")
        .body(page);
}

#[get("/equipment-item/update/{item_id}")]
async fn form_update_equipment_item(
    item_id: web::Path<i32>,
    data: web::Data<AppState>
) -> HttpResponse {
    let item = web::block(move || {
        use schema::equipment_item::dsl::*;

        let mut connection = establish_connection();

        return equipment_item
            .filter(id.eq(&item_id.to_owned()))
            .first::<EquipmentItem>(&mut connection)
            .unwrap()
    }).await.unwrap();

    let mut context = Context::new();
    context.insert("item_id", &item.id);
    context.insert("serial", &item.serial);
    let page = data.tera.render(
        "partials/warehouse/item-update.html", 
        &context).unwrap();

    return HttpResponse::Ok()
        .content_type("text/html")
        .body(page);
}

#[get("/equipment-items/{type_id}")]
async fn search_equipment_items(
    type_id: web::Path<i32>,
    data: web::Data<AppState>,
) -> HttpResponse {
    let items = web::block(move || {
        use schema::equipment_item::dsl::{
            equipment_item, type_id as type_id_col
        };

        let mut connection = establish_connection();

        return equipment_item
            .filter(type_id_col.eq(&type_id.to_owned()))
            .load::<EquipmentItem>(&mut connection)
            .unwrap()
    }).await.unwrap();

    let mut context = Context::new();
    context.insert("items", &items);
    let page = data.tera.render(
        "partials/warehouse/item-list.html", 
        &context).unwrap();

    HttpResponse::Ok()
        .content_type("text/html")
        .body(page)
}

// Types

#[get("/equipment-type/remove/{type_id}")]
async fn form_remove_equipment_type(
    type_id: web::Path<i32>,
    data: web::Data<AppState>
) -> HttpResponse {
    let mut context = Context::new();
    context.insert("type_id", &&type_id.to_owned());
    let page = data.tera.render(
        "partials/warehouse/type-delete.html", 
        &context).unwrap();

    return HttpResponse::Ok()
        .content_type("text/html")
        .body(page);
}

#[get("/equipment-type/new")]
async fn form_new_equipment_type(
    app: web::Data<AppState>
) -> HttpResponse {
    let context = Context::new();
    let page = app.tera.render(
        "partials/warehouse/type-new.html", 
        &context).unwrap();

    return HttpResponse::Ok()
        .content_type("text/html")
        .body(page);
}

#[get("/equipment-type/select/{type_id}")]
async fn get_equipment_type_select(
    type_id: web::Path<i32>,
    data: web::Data<AppState>,
) -> HttpResponse {
    let info = web::block(move || {
        let mut connection = establish_connection();
        use schema::equipment_type::dsl::{
            equipment_type, id as t_id
        };

        return equipment_type
            .filter(t_id.eq(type_id.to_owned()))
            .first::<EquipmentType>(&mut connection)
            .unwrap();
    }).await.unwrap();

    let mut context = Context::new();
    context.insert("id", &info.id);
    context.insert("name", &info.name);
    let page = data.tera.render(
        "partials/warehouse/type-select.html", 
        &context).unwrap();

    HttpResponse::Ok()
        .content_type("text/html")
        .body(page)
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct EquipmentTypeSearch {
    query: Option<String>,
}

#[get("/equipment-types")]
async fn search_equipment_types(
    query: web::Query<EquipmentTypeSearch>,
    data: web::Data<AppState>,
) -> HttpResponse {
    let query = query.query.clone().unwrap_or_default();

    let types = web::block(move || {
        use schema::equipment_type::dsl::*;

        let mut connection = establish_connection();

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
    }).await.unwrap();

    let mut context = Context::new();
    context.insert("types", &types);
    let page = data.tera.render(
        "partials/warehouse/type-list.html", 
        &context).unwrap();

    HttpResponse::Ok()
        .content_type("text/html")
        .body(page)
}

//
// Fragments
//

//
// Actions
//

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
struct EquipmentTypeDeletion {
    id: i32,
}

#[post("/equipment-type/delete")]
async fn delete_equipment_type(
    form: web::Form<EquipmentTypeDeletion>,
    data: web::Data<AppState>
) -> HttpResponse {
    let result = web::block(move || {
        let mut connection = establish_connection();
        use schema::equipment_type::dsl::*;

        delete(equipment_type)
            .filter(id.eq(&form.id))
            .execute(&mut connection)
    }).await.unwrap();

    let mut context = Context::new();
    let page = match result {
        Ok(_) => {
            context.insert("success", "Type was deleted.");
            data.tera.render("partials/success.html", &context).unwrap()
        }

        Err(_) => {
            context.insert("error", "Type could not be deleted.");
            data.tera.render("partials/error.html", &context).unwrap()
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

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
struct EquipmentItemDeletion {
    id: i32,
}

#[post("/equipment-item/delete")]
async fn delete_equipment_item(
    form: web::Form<EquipmentItemDeletion>,
    data: web::Data<AppState>
) -> HttpResponse {
    let result = web::block(move || {
        let mut connection = establish_connection();
        use schema::equipment_item::dsl::*;

        delete(equipment_item)
            .filter(id.eq(&form.id))
            .execute(&mut connection)
    }).await.unwrap();

    let mut context = Context::new();
    let page = match result {
        Ok(_) => {
            context.insert("success", "Item was deleted.");
            data.tera.render("partials/success.html", &context).unwrap()
        }

        Err(_) => {
            context.insert("error", "Item could not be deleted.");
            data.tera.render("partials/error.html", &context).unwrap()
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

#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
struct EquipmentTypeForm {
    type_name: String,
}

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
            .append_header(("HX-Trigger", "equipmentTypesChanged"))
            .content_type("text/html")
            .body(page),

        Err(_) => HttpResponse::UnprocessableEntity()
            .content_type("text/html")
            .body(page),
    }
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
struct EquipmentItemForm {
    type_id: i32,
    serial: String,
}

#[post("/equipment-item/new")]
async fn create_equipment_item(
    form: web::Form<EquipmentItemForm>,
    data: web::Data<AppState>
) -> HttpResponse {
    let result = web::block(move || {
        use schema::equipment_item::dsl::*;

        let mut connection = establish_connection();

        insert_into(equipment_item)
            .values((serial.eq(&form.serial), type_id.eq(&form.type_id)))
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
            .append_header(("HX-Trigger", "equipmentTypesChanged"))
            .content_type("text/html")
            .body(page),

        Err(_) => HttpResponse::UnprocessableEntity()
            .content_type("text/html")
            .body(page),
    }
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
struct EquipmentItemUpdate {
    item_id: i32,
    serial: String,
}

#[post("/equipment-item/update")]
async fn update_equipment_item(
    form: web::Form<EquipmentItemUpdate>,
    data: web::Data<AppState>
) -> HttpResponse {
    let result = web::block(move || {
        let mut connection = establish_connection();
        use schema::equipment_item::dsl::*;

        return update(equipment_item.filter(id.eq(&form.item_id)))
            .set(serial.eq(&form.serial))
            .execute(&mut connection);
    }).await.unwrap();

    let mut context = Context::new();
    let page = match result {
        Ok(_) => {
            context.insert("success", "Update succeeded.");
            data.tera.render("partials/success.html", &context).unwrap()
        }

        Err(_) => {
            context.insert("error", "Update failed.");
            data.tera.render("partials/error.html", &context).unwrap()
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

//
// Actions
//