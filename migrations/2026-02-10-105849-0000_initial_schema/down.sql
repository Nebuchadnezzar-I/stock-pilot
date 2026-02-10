-- Drop index first (depends on table)
DROP INDEX IF EXISTS dispatch_item_one_open_per_equipment;

-- Drop tables in reverse dependency order
DROP TABLE IF EXISTS dispatch_item;
DROP TABLE IF EXISTS order_line;
DROP TABLE IF EXISTS orders;
DROP TABLE IF EXISTS equipment_item;
DROP TABLE IF EXISTS equipment_type;
