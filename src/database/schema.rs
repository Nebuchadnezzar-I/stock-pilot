// @generated automatically by Diesel CLI.

diesel::table! {
    dispatch_item (id) {
        id -> Integer,
        order_line_id -> Integer,
        equipment_item_id -> Integer,
        loaded_at -> Integer,
        returned_at -> Nullable<Integer>,
        return_condition -> Nullable<Text>,
    }
}

diesel::table! {
    equipment_item (id) {
        id -> Integer,
        serial -> Text,
        type_id -> Integer,
    }
}

diesel::table! {
    equipment_type (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    order_line (id) {
        id -> Integer,
        order_id -> Integer,
        equipment_type_id -> Integer,
        equipment_count -> Integer,
    }
}

diesel::table! {
    orders (id) {
        id -> Integer,
        tag -> Text,
    }
}

diesel::joinable!(dispatch_item -> equipment_item (equipment_item_id));
diesel::joinable!(dispatch_item -> order_line (order_line_id));
diesel::joinable!(equipment_item -> equipment_type (type_id));
diesel::joinable!(order_line -> equipment_type (equipment_type_id));
diesel::joinable!(order_line -> orders (order_id));

diesel::allow_tables_to_appear_in_same_query!(
    dispatch_item,
    equipment_item,
    equipment_type,
    order_line,
    orders,
);
