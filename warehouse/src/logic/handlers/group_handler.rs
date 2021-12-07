#![allow(dead_code, unused_variables, unused_imports)]

use sqlx::Executor;

use crate::connection::pg_connection::get_pg_pool;
use crate::entities::{group, person};

pub async fn create(
    request: group::CreateGroupRequest,
) -> Result<group::CreateGroupResponse, Box<dyn std::error::Error>> {
    let pg_pool = get_pg_pool().await?;
    sqlx::query(
        r#"
        INSERT INTO groups(name)
        VALUES($1);
        "#,
    )
    .bind(&request.name)
    .execute(&pg_pool)
    .await?;

    sqlx::query(
        r#"
        INSERT INTO people_m2m_groups(people_name, group_name)
        VALUES($2, $1)
        "#,
    )
    .bind(&request.name)
    .bind(request.creator_username)
    .execute(&pg_pool)
    .await?;

    pg_pool.close().await;
    Ok(group::CreateGroupResponse {
        msg: "201".to_owned(),
    })
}

pub async fn delete(
    request: group::DeleteGroupRequest,
) -> Result<group::DeleteGroupResponse, Box<dyn std::error::Error>> {
    let pg_pool = get_pg_pool().await?;
    sqlx::query(
        r#"
        DELETE FROM groups WHERE name = $1;
        "#,
    )
    .bind(&request.group_name)
    .execute(&pg_pool)
    .await?;

    sqlx::query(
        r#"
        DELETE FROM people_m2m_groups WHERE group_name = $1;
        "#,
    )
    .bind(request.group_name)
    .execute(&pg_pool)
    .await?;
    pg_pool.close().await;
    Ok(group::DeleteGroupResponse {
        msg: "204".to_owned(),
    })
}

pub async fn remove_student_from_group(
    request: group::RemoveStudentFromGroupRequest,
) -> Result<group::RemoveStudentFromGroupResponse, Box<dyn std::error::Error>> {
    let pg_pool = get_pg_pool().await?;
    sqlx::query(
        r#"
        DELETE FROM people_m2m_groups 
        WHERE people_name = $1 AND group_name = $2
        "#,
    )
    .bind(request.student_name)
    .bind(request.group_name)
    .execute(&pg_pool)
    .await?;
    pg_pool.close().await;
    Ok(group::RemoveStudentFromGroupResponse {
        msg: "204".to_owned(),
    })
}

pub async fn add_student_to_group(
    request: group::AddStudentToGroupRequest,
) -> Result<group::AddStudentToGroupResponse, Box<dyn std::error::Error>> {
    let pg_pool = get_pg_pool().await?;
    sqlx::query(
        r#"
        INSERT INTO people_m2m_groups (people_name, group_name)
        VALUES($1, $2)
        "#,
    )
    .bind(request.student_name)
    .bind(request.group_name)
    .execute(&pg_pool)
    .await?;

    pg_pool.close().await;

    Ok(group::AddStudentToGroupResponse {
        msg: "201".to_owned(),
    })
}

pub async fn get_all_groups_by_username(
    request: group::GetAllGroupsByUsernameRequest,
) -> Result<group::GetAllGroupsByUsernameResponse, Box<dyn std::error::Error>> {
    let pg_pool = get_pg_pool().await?;
    let pg_pool = get_pg_pool().await?;
    let groups: Vec<group::FullGroup> = sqlx::query_as::<_, group::FullGroup>(
        r#"
        SELECT * FROM groups g
        INNER JOIN people_m2m_groups pmg ON g.name = pmg.group_name
        WHERE pmg.people_name = $1
        "#,
    )
    .bind(request.username)
    .fetch_all(&pg_pool)
    .await?;

    pg_pool.close().await;

    Ok(group::GetAllGroupsByUsernameResponse {
        group_names: groups.into_iter().map(|x| x.name).collect(),
    })
}

pub async fn get_all_students_by_group_name(
    request: group::GetAllStudentsByGroupNameRequest,
) -> Result<group::GetAllStudentsByGroupNameResponse, Box<dyn std::error::Error>> {
    let pg_pool = get_pg_pool().await?;
    let pg_pool = get_pg_pool().await?;
    let studs: Vec<person::FullPerson> = sqlx::query_as::<_, person::FullPerson>(
        r#"
        SELECT * FROM people p
        INNER JOIN people_m2m_groups pmg on p.username = pmg.people_name
        WHERE pmg.group_name = $1 AND p.is_teacher = false
        "#,
    )
    .bind(request.group_name)
    .fetch_all(&pg_pool)
    .await?;

    pg_pool.close().await;

    Ok(group::GetAllStudentsByGroupNameResponse {
        studs: studs
            .into_iter()
            .map(|x| group::get_all_students_by_group_name_response::Stud {
                username: x.username,
                is_teacher: x.is_teacher,
                password: String::from(""),
                salt: String::from(""),
            })
            .collect(),
    })
}
