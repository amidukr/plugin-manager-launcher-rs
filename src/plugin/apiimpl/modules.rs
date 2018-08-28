use std::sync::Arc;

use plugin::manager::*;

use plugin::api::plugin::*;
use plugin::api::modules::*;

impl PluginManagerModules for Arc<PluginManagerEngine> 
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
