extern crate plugin_launcher;

use plugin_launcher::plugin::api::modules::*;
use plugin_launcher::plugin::data::modules::*;
use plugin_launcher::plugin::helpers::plugin::*;
use plugin_launcher::plugin::api::container::*;

use plugin_launcher::plugin::helpers::langutils::*;

struct MockPlugin;

impl PluginEventsHandler for MockPlugin {

    fn register_components(&mut self, plugin_container: &PluginContainer) {
        panic!("Not supported");
    }

    fn unregister_components(&mut self, plugin_container: &PluginContainer) {
        panic!("Not supported");
    }

    fn start_plugin(&mut self) {
        panic!("Not supported");
    }

    fn stop_plugin(&mut self) {
        panic!("Not supported");
    }
}

#[test]
pub fn it_module_data_get_plugin() 
{
    let mut module_data = PluginManagerModulesData::new();

    module_data.add_module(
        PluginModuleHelper::new("module-1", "Module 1")
            .add_plugin("plugin-1", "Plugin 1", MockPlugin)
            .create()
    );

    if let PluginResult::ModuleNotFound(plugin_id) = module_data.get_plugin(&PluginId::new("bad-module-name", "plugin-1"))
    {
        assert_eq!(&PluginId::new("bad-module-name", "plugin-1"), plugin_id);
    }
    else
    {
        panic!("Module Not Found expected");
    }

    if let PluginResult::PluginNotFound(plugin_id) = module_data.get_plugin(&PluginId::new("module-1", "bad-plugin-name"))
    {
        assert_eq!(&PluginId::new("module-1", "bad-plugin-name"), plugin_id);
    }
    else
    {
        panic!("Plugin Not Found expected");
    }

    if let PluginResult::PluginFound(plugin) = module_data.get_plugin(&PluginId::new("module-1", "plugin-1"))
    {
        assert_eq!(&new_str("plugin-1"), plugin.get_plugin_name());
    }
    else
    {
        panic!("Plugin Found expected");
    }
}

#[test]
fn it_module_data_get_plugin_mut() 
{
    let mut module_data = PluginManagerModulesData::new();

    module_data.add_module(
        PluginModuleHelper::new("module-1", "Module 1")
            .add_plugin("plugin-1", "Plugin 1", MockPlugin)
            .create()
    );

    if let PluginResultMut::ModuleNotFound(plugin_id) = module_data.get_plugin_mut(&PluginId::new("bad-module-name", "plugin-1"))
    {
        assert_eq!(&PluginId::new("bad-module-name", "plugin-1"), plugin_id);
    }
    else
    {
        panic!("Module Not Found expected");
    }

    if let PluginResultMut::PluginNotFound(plugin_id) = module_data.get_plugin_mut(&PluginId::new("module-1", "bad-plugin-name"))
    {
        assert_eq!(&PluginId::new("module-1", "bad-plugin-name"), plugin_id);
    }
    else
    {
        panic!("Plugin Not Found expected");
    }

    if let PluginResultMut::PluginFound(plugin) = module_data.get_plugin_mut(&PluginId::new("module-1", "plugin-1"))
    {
        assert_eq!(&new_str("plugin-1"), plugin.get_plugin_name());
    }
    else
    {
        panic!("Plugin Found expected");
    }
}