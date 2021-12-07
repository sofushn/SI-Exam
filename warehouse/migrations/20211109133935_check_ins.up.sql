-- Add up migration script here
CREATE TABLE IF NOT EXISTS check_ins(
    id SERIAL PRIMARY KEY NOT NULL,
    check_start TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    check_end TIMESTAMP NOT NULL, 
    allowed_ip VARCHAR(255) NOT NULL,
    ip_salt VARCHAR(255) NOT NULL,
    code VARCHAR(30) NOT NULL
);
