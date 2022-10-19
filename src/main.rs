use std::sync::Arc;

mod app;
mod db;
mod http;
mod operations;
mod settings;

#[tokio::main]
async fn main() {
    let settings_reader = crate::settings::SettingsReader::new(".db-sync").await;

    let app = app::AppContext::new(Arc::new(settings_reader)).await;
    let app = Arc::new(app);

    crate::http::start_up::setup_server(app.clone(), 8000);

    app.app_states.wait_until_shutdown().await;
}
