use std::sync::Arc;

use plugin::manager::*;

use plugin::api::plugin::*;
use plugin::api::modules::*;
use plugin::utils::modules::*;

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

        let ref mut modules = self.modules_write_lock();
        let ref mut status  = self.status_write_lock();
        
        {
            let plugins_to_stop = PluginModulesUtils::get_plugins_for_configuration(modules, configuration, PluginActionEnum::Stop);

            for plugin_to_stop in plugins_to_stop {
                plugin_to_stop.stop_plugin();
                plugin_to_stop.unregister_components(self.get_plugin_container());
                
                status.set_plugin_status(plugin_to_stop, PluginStatusEnum::Inactive)
            }
        }

        {
            let plugins_to_start = PluginModulesUtils::get_plugins_for_configuration(modules, configuration, PluginActionEnum::Start);

            for plugin_to_start in plugins_to_start {
                plugin_to_start.register_components(self.get_plugin_container());
                plugin_to_start.start_plugin();

                status.set_plugin_status(plugin_to_start, PluginStatusEnum::Active)
            }
        }
        
        return Result::Ok(());
    }
}
