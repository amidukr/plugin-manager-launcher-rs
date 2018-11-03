extern crate plugin_launcher;

use plugin_launcher::plugin::api::modules::*;

#[test]
pub fn it_get_list_of_plugin_id_for_action_test() 
{
    let mut plugin_configuration = PluginConfiguration::new()
                                    .stop_plugin("module-1", "plugin-1")
                                    .stop_plugin("module-1", "plugin-2")
                                    .stop_plugin("module-2", "plugin-3")
                                    .start_plugin("module-2", "plugin-4")
                                    .start_plugin("module-3", "plugin-5");

    assert_eq!(vec![&PluginId::new("module-1", "plugin-1"),
                    &PluginId::new("module-1", "plugin-2"),
                    &PluginId::new("module-2", "plugin-3")
                ], 
    plugin_configuration.get_list_of_plugin_id_for_action(PluginActionEnum::Stop));

    assert_eq!(vec![&PluginId::new("module-2", "plugin-4"),
                    &PluginId::new("module-3", "plugin-5")
                ], 
    plugin_configuration.get_list_of_plugin_id_for_action(PluginActionEnum::Start));
}
