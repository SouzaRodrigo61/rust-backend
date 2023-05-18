use std::env;

use tracing::info;

use crate::settings::SETTINGS;

pub fn setup() {
  info!("Setting up logger");

  if env::var_os("RUST_LOG").is_none() {
    let level = SETTINGS.logger.level.as_str();
    let env = format!("rust_backend={level},tower_http={level}");

    env::set_var("RUST_LOG", env);
  }

  tracing_subscriber::fmt::init();
}
