PRAGMA foreign_keys = ON;

create table equipment_type (
    id integer not null constraint equipment_type_pk primary key autoincrement,
    name text not null
);

create table equipment_item (
    id integer not null constraint equipment_item_pk primary key autoincrement,
    serial text not null,
    type_id integer not null constraint equipment_item_type_id_fk references equipment_type
);

create table orders (
    id integer not null constraint orders_pk primary key autoincrement,
    tag text not null default (substr(lower(hex(randomblob(2))), 1, 4))
);

create table order_line (
    id integer not null constraint order_line_pk primary key autoincrement,
    order_id integer not null constraint order_line_order_id_fk references orders,
    equipment_type_id integer not null constraint order_line_equipment_type_id_fk references equipment_type,
    equipment_count integer not null
);

create table dispatch_item (
    id INTEGER primary key autoincrement,
    order_line_id INTEGER not null references order_line,
    equipment_item_id INTEGER not null references equipment_item,
    loaded_at INTEGER not null,
    returned_at INTEGER,
    return_condition TEXT,

    check (
        return_condition IN ('ok', 'damaged', 'missing', 'lost')
    )
);

create unique index dispatch_item_one_open_per_equipment on dispatch_item (equipment_item_id)
where
returned_at IS NULL;
