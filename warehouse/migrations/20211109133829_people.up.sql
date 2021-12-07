-- Add up migration script here
CREATE TABLE IF NOT EXISTS people(
    username VARCHAR(255) PRIMARY KEY UNIQUE NOT NULL,
    is_teacher BOOLEAN NOT NULL,
    pwd VARCHAR(255) NOT NULL,
    salt VARCHAR(255) NOT NULL
)


