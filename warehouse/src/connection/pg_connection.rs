use crate::utils::config::{is_production_mode, is_testing_mode, CONFIG};
use sqlx::postgres::PgPool;

// Mode control
lazy_static! {
    static ref DATABASE_URL: String = {
        if is_production_mode() {
            format!(
                "postgres://{}:{}@{}:{}/{}",
                CONFIG.production.database.user,
                CONFIG.production.database.pass,
                CONFIG.production.database.host,
                CONFIG.production.database.port,
                CONFIG.production.database.db,
            )
        } else if is_testing_mode() {
            format!(
                "postgres://{}:{}@{}:{}/{}",
                CONFIG.testing.database.user,
                CONFIG.testing.database.pass,
                CONFIG.testing.database.host,
                CONFIG.testing.database.port,
                CONFIG.testing.database.db,
            )
        } else {
            format!(
                "postgres://{}:{}@{}:{}/{}",
                CONFIG.development.database.user,
                CONFIG.development.database.pass,
                CONFIG.development.database.host,
                CONFIG.development.database.port,
                CONFIG.development.database.db,
            )
        }
    };
}

pub async fn get_pg_pool() -> anyhow::Result<PgPool> {
    Ok(PgPool::connect(&DATABASE_URL).await?)
}
