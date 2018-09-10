
use plugin::data::modules::*;

use plugin::api::error::PluginManagerError;
use plugin::api::modules::*;
use plugin::api::plugin::*;

pub struct PluginModulesUtils;

impl PluginModulesUtils {
    pub fn get_plugins_for_configuration<'a>(
        modules: &'a mut PluginManagerModulesData, 
        configuration: &PluginConfiguration, 
        action: PluginActionEnum) -> Result<Vec<&'a mut Plugin>, PluginManagerError>{
            
        panic!("Not supported");
    }
}
