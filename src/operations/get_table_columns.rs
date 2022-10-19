use std::collections::BTreeMap;

use crate::{app::AppContext, db::TableColumn};

pub struct TableSchema {
    pub columns: BTreeMap<String, BTreeMap<String, TableColumn>>,
    pub indexes: BTreeMap<String, BTreeMap<String, String>>,
}

impl TableSchema {
    pub fn new() -> Self {
        Self {
            columns: BTreeMap::new(),
            indexes: BTreeMap::new(),
        }
    }
}

pub async fn get_table_schema(app: &AppContext, table_name: &str) -> Result<TableSchema, String> {
    let mut result = TableSchema::new();
    for (env, postgres) in &app.postgress {
        let columns = postgres.get_columns(&table_name).await;
        if let Err(err) = columns {
            return Err(format!(
                "Can not columns data for env{}. Err:{:?}",
                env, err
            ));
        }
        let indexes = postgres.get_indexes(&table_name).await;

        if let Err(err) = indexes {
            return Err(format!(
                "Can not read indexes for env{}. Err:{:?}",
                env, err
            ));
        }

        for (column_name, column) in columns.unwrap() {
            if !result.columns.contains_key(&column_name) {
                result.columns.insert(column_name.clone(), BTreeMap::new());
            }

            if let Some(columns_schema) = result.columns.get_mut(&column_name) {
                columns_schema.insert(env.to_string(), column);
            }
        }

        for (index_name, index_value) in indexes.unwrap() {
            if !result.indexes.contains_key(&index_name) {
                result.indexes.insert(index_name.clone(), BTreeMap::new());
            }

            if let Some(index_schema) = result.indexes.get_mut(&index_name) {
                index_schema.insert(env.to_string(), index_value);
            }
        }
    }

    Ok(result)
}
