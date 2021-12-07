-- Add up migration script here
CREATE TABLE IF NOT EXISTS groups_m2m_check_ins(
    group_name VARCHAR(255) NOT NULL,
    check_in_id INTEGER NOT NULL
)