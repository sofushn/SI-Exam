use crate::connection::pg_connection::get_pg_pool;


// Just for testing
pub async fn truncate_table(table_name: &str) {
    let query_str = format!("TRUNCATE {}", table_name);
    sqlx::query(query_str.as_str())
        .execute(
            &get_pg_pool()
                .await
                .expect("Could not get pool for truncate table"),
        )
        .await
        .expect(format!("Could not execute truncate table {}", table_name).as_str());
}

