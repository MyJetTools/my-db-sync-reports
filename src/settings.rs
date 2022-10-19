use serde::{Deserialize, Serialize};

#[derive(my_settings_reader::SettingsModel, Serialize, Deserialize, Debug, Clone)]
pub struct SettingsModel {
    pub postgres_conn_string: Vec<PostgresSettings>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PostgresSettings {
    pub env: String,
    pub conn_string: String,
    pub scheme: String,
}
