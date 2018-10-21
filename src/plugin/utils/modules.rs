
use plugin::data::modules::*;
use plugin::data::status::*;

use plugin::api::error::*;
use plugin::api::modules::*;
use plugin::api::plugin::*;

pub struct PluginModulesUtils;

impl PluginModulesUtils {
    pub fn validate_configuration(
        modules: &PluginManagerModulesData, 
        configuration: &PluginConfiguration, 
        error_log: &mut ErrorLog) 
    {
        panic!("TODO: to implement");
    }

    pub fn validate_and_get_plugins_for_action<'a>(
        modules: &'a mut PluginManagerModulesData, 
        status: &PluginManagerStatusData, 
        configuration: &PluginConfiguration, 
        plugin_action: PluginActionEnum,
        error_log: &mut ErrorLog) -> Vec<(&'a PluginId, &'a mut Plugin)> 
    {
        panic!("TODO: to implement");
    }
}
