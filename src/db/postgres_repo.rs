use std::{collections::BTreeMap, sync::Arc};

use my_postgres::{MyPostgres, PostgressSettings};

use super::dto::*;

pub struct PostgresRepoSettings {
    pub conn_string: String,
}

impl PostgresRepoSettings {
    pub fn new(conn_string: String) -> Self {
        Self { conn_string }
    }
}

#[async_trait::async_trait]
impl PostgressSettings for PostgresRepoSettings {
    async fn get_connection_string(&self) -> String {
        self.conn_string.clone()
    }
}

pub struct PostgresRepo {
    postgress: MyPostgres,
    schema: String,
}

impl PostgresRepo {
    pub async fn new<TPostgressSettings: PostgressSettings + Sync + Send + 'static>(
        schema: String,
        postgres_settings: Arc<TPostgressSettings>,
    ) -> Self {
        let postgress = MyPostgres::new(crate::app::APP_NAME.to_string(), postgres_settings).await;
        Self { postgress, schema }
    }

    pub async fn get_list_of_tables(&self) -> Vec<String> {
        let sql = "SELECT * FROM information_schema.tables where table_schema = $1";

        let response: Vec<TableNameDto> = self
            .postgress
            .query_rows(sql.to_string(), &[&self.schema])
            .await
            .unwrap();

        let mut result = Vec::with_capacity(response.len());

        for item in response {
            if item.table_name.starts_with("pg") {
                continue;
            }
            result.push(item.table_name);
        }

        result
    }

    pub async fn get_columns(&self, table_name: &str) -> BTreeMap<String, TableColumn> {
        let sql =
            "SELECT * FROM information_schema.columns WHERE table_schema = $1 AND table_name = $2";

        let response: Vec<TableColumn> = self
            .postgress
            .query_rows(sql.to_string(), &[&self.schema, &table_name])
            .await
            .unwrap();

        let mut result = BTreeMap::new();

        for column in response {
            result.insert(column.column_name.to_string(), column);
        }

        result
    }

    pub async fn get_indexes(&self, table_name: &str) -> BTreeMap<String, String> {
        let sql =
            "SELECT  \"indexname\", \"indexdef\" FROM pg_indexes WHERE schemaname = $1 AND tablename = $2";

        let response: Vec<TableIndex> = self
            .postgress
            .query_rows(sql.to_string(), &[&self.schema, &table_name])
            .await
            .unwrap();

        let mut result = BTreeMap::new();

        for index in response {
            result.insert(index.indexname, index.indexdef);
        }

        result
    }
}
