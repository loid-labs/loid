use crate::database::base::SQLResource;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::{MySql, Pool};

pub struct MySQLResource {
    pool: Pool<MySql>,
}

impl SQLResource for MySQLResource {
    async fn execute_query(&self) {
        todo!()
    }

    async fn ping(&self) {
        let pool = MySqlPoolOptions::new()
            .max_connections(1)
            .connect("")
            .await
            .unwrap();
        todo!()
    }
}
