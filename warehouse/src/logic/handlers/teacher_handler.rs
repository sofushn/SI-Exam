#![allow(dead_code, unused_variables, unused_imports)]

use sqlx::Executor;

use crate::connection::pg_connection::get_pg_pool;
use crate::entities::{person, check_in};
use crate::logic::cryptography::{generate_code, generate_salt, hash_and_salt};
use crate::utils::time::{get_current_time_and_add_request_millis, parse_request_time};

pub async fn generate_code_and_start(
    request: person::GenerateCodeAndStartRequest,
) -> Result<person::GenerateCodeAndStartResponse, Box<dyn std::error::Error>> {
    let pg_pool = get_pg_pool().await?;
    let code = generate_code();
    let salt = generate_salt();

    sqlx::query(
        r#"
        INSERT INTO check_ins(check_end, allowed_ip, ip_salt, code)
        VALUES($1, $2, $3, $4)
    "#,
    )
    .bind(get_current_time_and_add_request_millis(&request))
    .bind(hash_and_salt(request.ip_encrypted, &salt)?)
    .bind(format!("{:?}", &salt))
    .bind(&code)
    .execute(&pg_pool)
    .await?;

    let check_in = sqlx::query_as::<_,check_in::CheckIn>(
        r#"
        SELECT * FROM check_ins
        WHERE code = $1
        "#
    )
    .bind(&code)
    .fetch_one(&pg_pool)
    .await?;

    sqlx::query(
        r#"
        INSERT INTO groups_m2m_check_ins(group_name, check_in_id)
        VALUES($1, $2)
        "#
    )
    .bind(request.group_name)
    .bind(check_in.id)
    .execute(&pg_pool)
    .await?;


    pg_pool.close().await;
    Ok(person::GenerateCodeAndStartResponse { code: code })
}

pub async fn edit_countdown(
    request: person::EditCountdownRequest,
) -> Result<person::EditCountdownResponse, Box<dyn std::error::Error>> {
    let pg_pool = get_pg_pool().await?;

    sqlx::query(
        r#"
        UPDATE check_ins SET check_end = $1
        WHERE code = $2
        "#,
    )
    .bind(parse_request_time(request.date_time.as_str()))
    .bind(request.code)
    .execute(&pg_pool)
    .await?;

    pg_pool.close().await;
    Ok(person::EditCountdownResponse {
        msg: "Ok".to_owned(),
    })
}

pub async fn delete_code(
    request: person::DeleteCodeRequest,
) -> Result<person::DeleteCodeResponse, Box<dyn std::error::Error>> {
    let pg_pool = get_pg_pool().await?;
    sqlx::query(
        r#"
        DELETE FROM check_ins where code = $1
        "#,
    )
    .bind(request.code)
    .execute(&pg_pool)
    .await?;

    pg_pool.close().await;
    Ok(person::DeleteCodeResponse {
        msg: "Ok".to_owned(),
    })
}
