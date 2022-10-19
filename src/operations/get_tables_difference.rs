use std::{collections::BTreeMap, sync::Arc};

use tokio::sync::Mutex;

use crate::{app::AppContext, db::TableColumn};

pub struct TableInfo {
    pub columns: BTreeMap<String, BTreeMap<String, TableColumn>>,
    pub indexes: BTreeMap<String, BTreeMap<String, String>>,
}

impl TableInfo {
    pub fn new() -> Self {
        Self {
            columns: BTreeMap::new(),
            indexes: BTreeMap::new(),
        }
    }
}

pub async fn get_tables_difference(app: &AppContext) -> BTreeMap<String, TableInfo> {
    let result = BTreeMap::new();

    let result = Arc::new(Mutex::new(Some(result)));

    let mut futures = Vec::new();

    for (env, postgres) in &app.postgress {
        let tables = postgres.get_list_of_tables().await;
        let result = result.clone();
        let postgres_moved = postgres.clone();

        let env = env.to_string();

        let handle = tokio::spawn(async move {
            for table_name in tables {
                let columns = postgres_moved.get_columns(&table_name).await;
                let indexes = postgres_moved.get_indexes(&table_name).await;

                let mut write_access = result.lock().await;

                let result_access = write_access.as_mut().unwrap();
                if !result_access.contains_key(&table_name) {
                    result_access.insert(table_name.clone(), TableInfo::new());
                }

                if let Some(table_info) = result_access.get_mut(&table_name) {
                    table_info.columns.insert(env.clone(), columns);
                    table_info.indexes.insert(env.clone(), indexes);
                }
            }
        });

        futures.push(handle);
    }

    for future in futures {
        future.await.unwrap();
    }

    let result = { result.lock().await.take().unwrap() };

    result
}
