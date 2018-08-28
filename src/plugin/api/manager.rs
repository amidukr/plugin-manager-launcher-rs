use std::sync::Arc;

use plugin::api::modules::PluginManagerModules;

pub trait PluginManager
{
    fn get_plugin_manager(&self) -> Arc<PluginManager>;
    fn get_plugin_modules(&self) -> Arc<PluginManagerModules>;
}
