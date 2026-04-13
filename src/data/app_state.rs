use crate::data::config::InterfaceConfig;
use std::sync::Arc;

pub struct AppState {
    pub interfaces: Arc<InterfaceConfig>,
}
