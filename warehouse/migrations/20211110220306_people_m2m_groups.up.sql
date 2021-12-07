-- Add up migration script here
CREATE TABLE IF NOT EXISTS people_m2m_groups(
    people_name VARCHAR(255) NOT NULL,
    group_name VARCHAR(255) NOT NULL
)



