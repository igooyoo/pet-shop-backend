--
--  Drop PetsShop Aggregations functions and triggers
--

-- user_aggregations_orders
DROP TRIGGER user_aggregations_user_orders ON user_orders;
DROP FUNCTION IF EXISTS user_aggregations_user_orders();
-- category_aggregations_products
DROP TRIGGER category_aggregations_products ON products;
DROP FUNCTION IF EXISTS category_aggregations_products();
