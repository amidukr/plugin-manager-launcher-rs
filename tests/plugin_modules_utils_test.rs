extern crate plugin_launcher;

use std::result::Result;

use plugin_launcher::plugin::utils::modules::PluginModulesUtils;

use plugin_launcher::plugin::helpers::plugin::*;
use plugin_launcher::plugin::helpers::langutils::*;

use plugin_launcher::plugin::data::modules::*;

use plugin_launcher::plugin::api::error::PluginManagerError;
use plugin_launcher::plugin::api::modules::*;
use plugin_launcher::plugin::api::plugin::*;
use plugin_launcher::plugin::api::container::*;


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
fn it_get_plugins_for_configuration_test() {
    let mut modules = PluginManagerModulesData::new();
    
    modules.add_module(PluginModuleHelper::new("mock-module-1", "Mock module for unit testing")
                        .add_plugin("mock-plugin-1", "Mock plugin 1", MockPlugin)
                        .add_plugin("mock-plugin-2", "Mock plugin 2", MockPlugin)
                        .create());

    modules.add_module(PluginModuleHelper::new("mock-module-2", "Mock module for unit testing")
                        .add_plugin("mock-plugin-3", "Mock plugin 3", MockPlugin)
                        .add_plugin("mock-plugin-4", "Mock plugin 4", MockPlugin)
                        .add_plugin("mock-plugin-5", "Mock plugin 5", MockPlugin)
                        .create());

    modules.add_module(PluginModuleHelper::new("mock-module-3", "Mock module for unit testing")
                        .add_plugin("mock-plugin-6", "Mock plugin 6", MockPlugin)
                        .add_plugin("mock-plugin-7", "Mock plugin 7", MockPlugin)
                        .create());


    let plugin_configuration = PluginConfiguration::new()
                                    .stop_plugin("mock-module-1", "mock-plugin-1")
                                    .stop_plugin("mock-module-1", "mock-plugin-2")
                                    .stop_plugin("mock-module-2", "mock-plugin-3")
                                    .stop_plugin("mock-module-2", "mock-plugin-4")
                                    .start_plugin("mock-module-1", "mock-plugin-1")
                                    .start_plugin("mock-module-1", "mock-plugin-2")
                                    .start_plugin("mock-module-2", "mock-plugin-3")
                                    .start_plugin("mock-module-2", "mock-plugin-5")
                                    .start_plugin("mock-module-6", "mock-plugin-6");

    panic!("Assert not implemented!");
}


#[test]
fn it_get_plugins_for_configuration_not_found_test() {
    let mut modules = PluginManagerModulesData::new();
    
    modules.add_module(PluginModuleHelper::new("mock-module-1", "Mock module for unit testing")
                        .add_plugin("mock-plugin-1", "Mock plugin 1", MockPlugin)
                        .add_plugin("mock-plugin-2", "Mock plugin 2", MockPlugin)
                        .create());


    let plugin_configuration_wrong_plugin = PluginConfiguration::new().stop_plugin("mock-module-1", "mock-plugin-3");
    let plugin_configuration_wrong_module = PluginConfiguration::new().stop_plugin("mock-module-2", "mock-plugin-1");
}
