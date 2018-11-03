
use plugin::data::modules::*;
use plugin::data::status::*;

use plugin::api::error::*;
use plugin::api::modules::*;
use plugin::api::plugin::*;

use std::mem::transmute;

use std::collections::HashSet;

pub struct PluginModulesUtils;

impl PluginModulesUtils {
    pub fn validate_configuration(
        modules: &PluginManagerModulesData, 
        configuration: &PluginConfiguration, 
        error_log: &mut ErrorLog) 
    {
        for plugin_action in configuration.get_plugin_actions() {
            match modules.get_plugin(plugin_action.get_plugin_id()) {
                PluginResult::ModuleNotFound(plugin_id) => 
                    error_log.add_error(PluginManagerError::ModuleNotFound(plugin_id.get_module_name().clone())),
                PluginResult::PluginNotFound(plugin_id) =>
                    error_log.add_error(PluginManagerError::PluginNotFound(plugin_id.clone())),
                PluginResult::PluginFound(..) => continue
            }
        }
    }

    pub fn validate_and_get_plugins_for_action<'a, 'b>(
        modules: &'b mut PluginManagerModulesData, 
        status: &PluginManagerStatusData, 
        configuration: &'a PluginConfiguration, 
        plugin_action: PluginActionEnum,
        error_log: &mut ErrorLog) -> Vec<(&'a PluginId, &'b mut Plugin)> 
    {
        let ptr_modules = modules as *mut PluginManagerModulesData;

        let expected_status = match plugin_action 
        {
            PluginActionEnum::Start => PluginStatusEnum::Inactive,
            PluginActionEnum::Stop =>  PluginStatusEnum::Active
        };

        let mut plugin_duplicates_filter: HashSet<*mut Plugin> = HashSet::new();

        return configuration.get_list_of_plugin_id_for_action(plugin_action)
        .into_iter()
        .filter_map(|plugin_id| {
            unsafe {
                let mut_modules: &'b mut PluginManagerModulesData = transmute(ptr_modules);

                if let PluginResultMut::PluginFound(plugin) = mut_modules.get_plugin_mut(plugin_id) 
                {

                    let plugin_status = status.get_plugin_status(plugin_id);
                    if(plugin_status != expected_status) {
                        match plugin_action 
                        {
                            PluginActionEnum::Start => error_log.add_warning(PluginManagerError::PluginAlreadyStarted(plugin_id.clone())),
                            PluginActionEnum::Stop  => error_log.add_warning(PluginManagerError::PluginNotStarted(plugin_id.clone()))
                        };

                        return None;
                    }

                    if plugin_duplicates_filter.contains(&(plugin as *mut Plugin))
                    {
                        return None;
                    }

                    plugin_duplicates_filter.insert(plugin);

                    return Some((plugin_id, plugin))
                }
                else 
                {
                    return None
                }
            }

        })
        .collect();
    }
}
