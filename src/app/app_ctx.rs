use std::sync::Arc;

use rust_extensions::AppStates;

use crate::db::{PostgresRepo, PostgresRepoSettings};

pub const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const APP_NAME: &'static str = env!("CARGO_PKG_NAME");

pub struct AppContext {
    pub app_states: Arc<AppStates>,
    pub postgress: Vec<(String, Arc<PostgresRepo>)>,
}

impl AppContext {
    pub async fn new(settings_reader: Arc<crate::settings::SettingsReader>) -> Self {
        let settings = settings_reader.get_settings().await;

        let mut postgress = Vec::new();

        for settings in settings.postgres_conn_string {
            let postgres = PostgresRepo::new(
                settings.scheme,
                Arc::new(PostgresRepoSettings::new(settings.conn_string)),
            )
            .await;
            postgress.push((settings.env, Arc::new(postgres)));
        }

        Self {
            app_states: Arc::new(AppStates::create_initialized()),
            postgress,
        }
    }
}
