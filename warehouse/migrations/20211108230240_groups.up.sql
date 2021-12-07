-- Add up migration script here
CREATE TABLE IF NOT EXISTS groups(
    name VARCHAR(255) PRIMARY KEY UNIQUE NOT NULL
)
