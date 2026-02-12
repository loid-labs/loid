use crate::database::base::SQLResource;
use sqlx::{Executor, Pool, Postgres};

pub struct PostgresResource {
    pool: Pool<Postgres>,
}

impl SQLResource for PostgresResource {
    async fn execute_query(&self) {
        todo!()
    }

    async fn ping(&self) {
        let ping_res = sqlx::query("SELECT 1").fetch_one(&self.pool).await;
    }
}
