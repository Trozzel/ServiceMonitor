-- Your SQL goes here
SET FOREIGN_KEY_CHECKS = 0;

CREATE TABLE
  users (
    id BIGINT PRIMARY KEY AUTO_INCREMENT UNIQUE NOT NULL,
    name TEXT NOT NULL,
    password TEXT NOT NULL,
    group_accts_id BIGINT DEFAULT NULL,
    active BOOLEAN NOT NULL DEFAULT true,
    created DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    modified DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
      ON UPDATE CURRENT_TIMESTAMP,
    CONSTRAINT FOREIGN KEY fk_users_group_accts_id (group_accts_id)
      REFERENCES group_accts (id)
      ON DELETE SET NULL
  );
