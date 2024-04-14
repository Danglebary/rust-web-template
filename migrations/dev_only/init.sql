-- Brute Force DROP DB and user
DROP DATABASE IF EXISTS test_db;
DROP USER IF EXISTS test_user;

-- Create user and database
CREATE USER test_user IDENTIFIED BY 'test_password';
CREATE DATABASE test_db;
GRANT ALL PRIVILEGES ON test_db.* TO test_user IDENTIFIED BY 'test_password';