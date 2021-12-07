use chrono::NaiveDateTime;

#[derive(sqlx::FromRow, Clone)]
pub struct CheckIn {
    pub id: i32,
    pub check_start: NaiveDateTime,
    pub check_end: NaiveDateTime,
    pub allowed_ip: String,
    pub ip_salt: String,
    pub code: String,
}
