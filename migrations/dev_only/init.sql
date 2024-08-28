-- DEV ONLY - Brute Force DROP DB and user (for local dev and unit tests)
SELECT pg_terminate_backend(pid) FROM pg_stat_activity
WHERE usename = "test_user" OR datname = "test_db";
DROP TABLE IF EXISTS test_db;
DROP USER IF EXISTS test_user;

-- DEV ONLY - Create user and database (for local dev and unit tests)
CREATE USER test_user IDENTIFIED BY 'test_password';
CREATE DATABASE test_db;
GRANT ALL PRIVILEGES ON test_db.* TO test_user IDENTIFIED BY 'test_password';