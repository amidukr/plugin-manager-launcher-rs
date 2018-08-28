use plugin::api::modules::PluginManagerModules;

pub trait PluginManager
{
    fn get_plugin_modules(&self) -> &PluginManagerModules;
}