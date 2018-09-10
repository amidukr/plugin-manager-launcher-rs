use std::sync::Arc;

use plugin::api::modules::PluginId;

#[derive(Debug, PartialEq)]
pub enum PluginManagerError {
    ModuleNotFound(Vec<Arc<str>>),
    PluginNotFound(Vec<PluginId>)
}