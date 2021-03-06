use plugin::api::container::*;
use std::sync::Arc;

pub trait Plugin: Sync + Send {
    fn get_plugin_name(&self) -> &Arc<str>;
    fn get_plugin_short_description(&self) -> &Arc<str>;

    fn register_components(&mut self, plugin_container: &PluginContainer);
    fn unregister_components(&mut self, plugin_container: &PluginContainer);

    fn start_plugin(&mut self);
    fn stop_plugin(&mut self);
}

pub trait PluginModule: Sync + Send {
    fn get_module_name(&self) -> &Arc<str>;
    fn get_module_short_description(&self) -> &Arc<str>;

    fn get_plugins_names(&self) -> Arc<Vec<Arc<str>>>;
    fn get_plugin(&self, plugin_name: &Arc<str>) -> Option<&Plugin>;
    fn get_plugin_mut(&mut self, plugin_name: &Arc<str>) -> Option<&mut Plugin>;
}
