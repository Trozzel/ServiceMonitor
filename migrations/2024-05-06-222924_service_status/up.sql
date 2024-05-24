-- Your SQL goes here
SET FOREIGN_KEY_CHECKS = 0;

CREATE TABLE
  service_status (
    id BIGINT PRIMARY KEY AUTO_INCREMENT UNIQUE NOT NULL,
    hostname TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    enabled BOOLEAN,
    active_status TEXT,
    last_check DATETIME ON UPDATE CURRENT_TIMESTAMP
  );

SET FOREIGN_KEY_CHECKS = 1
