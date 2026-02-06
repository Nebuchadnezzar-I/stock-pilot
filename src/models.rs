use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use crate::schema::*;

#[derive(Debug, Queryable, Identifiable, Associations)]
#[diesel(table_name = dispatch_item)]
#[diesel(belongs_to(OrderLine))]
#[diesel(belongs_to(EquipmentItem))]
pub struct DispatchItem {
    pub id: i32,
    pub order_line_id: i32,
    pub equipment_item_id: i32,
    pub loaded_at: i64,
    pub returned_at: Option<i64>,
    pub return_condition: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = dispatch_item)]
pub struct NewDispatchItem {
    pub order_line_id: i32,
    pub equipment_item_id: i32,
    pub loaded_at: i32,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Queryable, Identifiable, Associations)]
#[diesel(table_name = equipment_item)]
#[diesel(belongs_to(EquipmentType, foreign_key = type_id))]
pub struct EquipmentItem {
    pub id: i32,
    pub serial: String,
    pub type_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = equipment_item)]
pub struct NewEquipmentItem<'a> {
    pub serial: &'a str,
    pub type_id: i32,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Queryable, Identifiable)]
#[diesel(table_name = equipment_type)]
pub struct EquipmentType {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[diesel(table_name = equipment_type)]
pub struct NewEquipmentType<'a> {
    pub name: &'a str,
}

#[derive(Debug, Queryable, Identifiable)]
#[diesel(table_name = orders)]
pub struct Order {
    pub id: i32,
    pub tag: String,
}

#[derive(Insertable)]
#[diesel(table_name = orders)]
pub struct NewOrder<'a> {
    pub tag: &'a str,
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[diesel(table_name = order_line)]
#[diesel(belongs_to(Order))]
#[diesel(belongs_to(EquipmentType))]
pub struct OrderLine {
    pub id: i32,
    pub order_id: i32,
    pub equipment_type_id: i32,
    pub equipment_count: i32,
}

#[derive(Insertable)]
#[diesel(table_name = order_line)]
pub struct NewOrderLine {
    pub order_id: i32,
    pub equipment_type_id: i32,
    pub equipment_count: i32,
}

