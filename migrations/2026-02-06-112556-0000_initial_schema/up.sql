PRAGMA foreign_keys = ON;

CREATE TABLE equipment_type (
    id   INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name TEXT NOT NULL
);

CREATE TABLE equipment_item (
    id      INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    serial  TEXT NOT NULL,
    type_id INTEGER NOT NULL,
    FOREIGN KEY (type_id)
        REFERENCES equipment_type(id)
);

CREATE TABLE orders (
    id  INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    tag TEXT NOT NULL
);

CREATE TABLE order_line (
    id                INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    order_id          INTEGER NOT NULL,
    equipment_type_id INTEGER NOT NULL,
    equipment_count   INTEGER NOT NULL,

    FOREIGN KEY (order_id)
        REFERENCES orders(id),

    FOREIGN KEY (equipment_type_id)
        REFERENCES equipment_type(id)
);

CREATE TABLE dispatch_item (
    id                INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    order_line_id     INTEGER NOT NULL,
    equipment_item_id INTEGER NOT NULL,
    loaded_at         INTEGER NOT NULL,
    returned_at       INTEGER,
    return_condition  TEXT CHECK (
        return_condition IN ('ok', 'damaged', 'missing', 'lost')
    ),

    FOREIGN KEY (order_line_id)
        REFERENCES order_line(id),

    FOREIGN KEY (equipment_item_id)
        REFERENCES equipment_item(id)
);

CREATE UNIQUE INDEX dispatch_item_one_open_per_equipment
ON dispatch_item (equipment_item_id)
WHERE returned_at IS NULL;
