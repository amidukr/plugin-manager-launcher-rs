use std::sync::Arc;

use plugin::api::plugin::*;
use plugin::api::modules::*;

pub struct PluginManagerModulesApi 
{

}

impl PluginManagerModulesApi {
    pub fn new() -> PluginManagerModulesApi 
    {
        return PluginManagerModulesApi{};
    }
}

impl PluginManagerModules for PluginManagerModulesApi
{
    fn add_external_module(&self, library_file_path: &Arc<str>) {
        panic!("Operation Unsupported yet");
    }

    fn add_module(&self, module: Box<PluginModule>) {
        panic!("Operation Unsupported yet");
    }

    fn get_status(&self) -> Arc<PluginConfigurationStatus> {
        panic!("Operation Unsupported yet");
    }

    fn apply_configuration(&self, configuration: &PluginConfiguration) -> Result<(), &str> {
        panic!("Operation Unsupported yet");
    }
}
