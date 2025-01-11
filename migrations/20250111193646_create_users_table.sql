-- Add migration script here
CREATE TABLE users (
                       id SERIAL PRIMARY KEY,
                       username VARCHAR(50) NOT NULL,
                       email VARCHAR(50) UNIQUE NOT NULL,
                       password VARCHAR(128) NOT NULL,
                       created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);