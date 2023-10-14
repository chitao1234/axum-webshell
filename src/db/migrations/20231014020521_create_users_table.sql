-- migrate:up
CREATE TABLE users (
    id BIGINT PRIMARY KEY, 
    password_hash VARCHAR NOT NULL,
    name VARCHAR NOT NULL
);

INSERT INTO users(id, password_hash, name) VALUES('1', 'test', 'chi');

-- migrate:down
drop table users;
