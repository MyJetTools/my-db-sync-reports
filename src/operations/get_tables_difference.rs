use std::{collections::BTreeMap, time::Duration};

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

pub async fn get_tables_difference(
    app: &AppContext,
) -> Result<BTreeMap<String, TableInfo>, String> {
    let mut result = BTreeMap::new();

    let mut futures = Vec::new();

    for (env, postgres) in &app.postgress {
        let tables = postgres.get_list_of_tables().await;

        if let Err(err) = &tables {
            return Err(format!("Can not read tables for env{}. Err:{:?}", env, err));
        }

        let tables = tables.unwrap();

        let postgres_moved = postgres.clone();
        let env = env.to_string();

        let handle = tokio::spawn(async move {
            let mut result = BTreeMap::new();
            for table_name in tables {
                let columns = tokio::time::timeout(
                    Duration::from_secs(10),
                    postgres_moved.get_columns(&table_name),
                )
                .await;

                if let Err(_) = columns {
                    return Err(format!("Can not columns data for env: {}. Timeout", env));
                }

                let columns = columns.unwrap();

                if let Err(err) = columns {
                    return Err(format!(
                        "Can not columns data for env: {}. Err:{:?}",
                        env, err
                    ));
                }

                let columns = columns.unwrap();

                let indexes = tokio::time::timeout(
                    Duration::from_secs(10),
                    postgres_moved.get_indexes(&table_name),
                )
                .await;

                if let Err(_) = indexes {
                    return Err(format!(
                        "Can not get indexes data for env: {}. Timeout",
                        env
                    ));
                }

                let indexes = indexes.unwrap();

                if let Err(err) = indexes {
                    return Err(format!(
                        "Can not read indexes for env: {}. Err:{:?}",
                        env, err
                    ));
                }

                let indexes = indexes.unwrap();

                result.insert(table_name.to_string(), (columns, indexes));
            }

            Ok((env, result))
        });

        futures.push(handle);
    }

    for future in futures {
        let (env, columns_and_indexes) = future.await.unwrap()?;

        for (table, (columns, indexes)) in columns_and_indexes {
            if !result.contains_key(&table) {
                result.insert(table.to_string(), TableInfo::new());
            }

            if let Some(table_info) = result.get_mut(&table) {
                table_info.columns.insert(env.to_string(), columns);
                table_info.indexes.insert(env.to_string(), indexes);
            }
        }
    }

    Ok(result)
}
