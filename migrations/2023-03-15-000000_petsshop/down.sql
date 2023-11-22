
--
--  Drop recruit Management.
--

DROP TABLE IF EXISTS users CASCADE;
DROP TABLE IF EXISTS identifies CASCADE;
DROP TABLE IF EXISTS user_orders CASCADE;
DROP TABLE IF EXISTS categories CASCADE;
DROP TABLE IF EXISTS products CASCADE;
DROP TABLE IF EXISTS order_items CASCADE;
DROP TABLE IF EXISTS payments CASCADE;
DROP TABLE IF EXISTS comments CASCADE;
DROP TABLE IF EXISTS cards CASCADE;

DROP TABLE IF EXISTS user_aggregations CASCADE;
DROP TABLE IF EXISTS category_aggregations CASCADE;

DROP FUNCTION IF EXISTS user_aggregations_main CASCADE;
DROP FUNCTION IF EXISTS category_aggregations_main CASCADE;
