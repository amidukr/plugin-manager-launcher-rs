use std::sync::Arc;

use plugin::manager::*;

use plugin::api::error::PluginManagerError;
use plugin::api::error::ErrorLog;
use plugin::api::plugin::*;
use plugin::api::modules::*;
use plugin::utils::modules::*;

use plugin::data::modules::*;

impl PluginManagerModules for Arc<PluginManagerEngine> 
{
    fn add_external_module(&self, library_file_path: &Arc<str>) {
        panic!("Operation Unsupported yet");
    }

    fn add_module(&self, module: Box<PluginModule>) {
        let ref mut modules = self.modules_write_lock();

        modules.add_module(module);
    }

    fn get_status(&self) -> Arc<PluginConfigurationStatus> {
        panic!("Operation Unsupported yet");
    }

    fn apply_configuration(&self, configuration: &PluginConfiguration) -> ErrorLog {

        let ref mut modules = self.modules_write_lock();
        let ref mut status  = self.status_write_lock();

        let mut error_log = ErrorLog::new();

        PluginModulesUtils::validate_configuration(modules, configuration, &mut error_log);

        if(error_log.is_failed()) {
            return error_log;
        }
        
        {
            let mut plugins_to_stop = PluginModulesUtils::validate_and_get_plugins_for_action(modules, status, configuration, PluginActionEnum::Stop, &mut error_log);

            for &mut (ref plugin_to_stop_id, ref mut plugin_to_stop) in &mut plugins_to_stop {
                plugin_to_stop.stop_plugin();
            }
            

            for &mut (ref plugin_to_stop_id, ref mut plugin_to_stop) in &mut plugins_to_stop {
                plugin_to_stop.unregister_components(self.get_plugin_container());
            }
        
            
            for &mut (ref plugin_to_stop_id, ref mut plugin_to_stop) in &mut plugins_to_stop {
                status.set_plugin_status(plugin_to_stop_id, PluginStatusEnum::Inactive)
            }
        
        }

        {
            let mut plugins_to_start = PluginModulesUtils::validate_and_get_plugins_for_action(modules, status, configuration, PluginActionEnum::Start, &mut error_log);

            for &mut (ref plugin_to_start_id, ref mut plugin_to_start) in &mut plugins_to_start {
                plugin_to_start.register_components(self.get_plugin_container());
            }

            for &mut (ref plugin_to_start_id, ref mut plugin_to_start) in &mut plugins_to_start {
                plugin_to_start.start_plugin();
            }

            for &mut (ref plugin_to_start_id, ref mut plugin_to_start) in &mut plugins_to_start {
                status.set_plugin_status(plugin_to_start_id, PluginStatusEnum::Active)
            }
        }
        
        return error_log;
    }
}
