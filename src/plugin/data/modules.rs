use std::sync::Arc;
use std::collections::HashMap;

use plugin::api::plugin::*;
use plugin::api::modules::PluginId;
use std::option::Option;

pub struct PluginManagerModulesData {
    modules: HashMap<Arc<str>, Box<PluginModule>>    
}

pub enum PluginResult<'a, 'b> {
    PluginFound(&'a Plugin),

    #[derive(Debug, PartialEq)]
    ModuleNotFound(&'b PluginId),

    #[derive(Debug, PartialEq)]
    PluginNotFound(&'b PluginId)
}

pub enum PluginResultMut<'a, 'b> {
    PluginFound(&'a mut Plugin),
    ModuleNotFound(&'b PluginId),
    PluginNotFound(&'b PluginId)
}

impl PluginManagerModulesData {
    pub fn new() -> PluginManagerModulesData {
        return PluginManagerModulesData{
            modules: HashMap::new()
        };
    }

    pub fn add_module(&mut self, module: Box<PluginModule>) {
        self.modules.insert(module.get_module_name().clone(), module);
    }

    pub fn get_plugin<'a, 'b>(&'a self, pluginId: &'b PluginId) -> PluginResult<'a, 'b> 
    {
        if let Some(module) = self.modules.get(pluginId.get_module_name())
        {
            if let Some(plugin) = module.get_plugin(pluginId.get_plugin_name()) 
            {
                return PluginResult::PluginFound(plugin);
            }
            else
            {
                return PluginResult::PluginNotFound(pluginId);
            }
        }
        else 
        {
            return PluginResult::ModuleNotFound(pluginId);
        }
    }

    pub fn get_plugin_mut<'a, 'b>(&'a mut self, pluginId: &'b PluginId) -> PluginResultMut<'a, 'b> 
    {
        if let Some(module) = self.modules.get_mut(pluginId.get_module_name())
        {
            if let Some(plugin) = module.get_plugin_mut(pluginId.get_plugin_name()) 
            {
                return PluginResultMut::PluginFound(plugin);
            }
            else
            {
                return PluginResultMut::PluginNotFound(pluginId);
            }
        }
        else 
        {
            return PluginResultMut::ModuleNotFound(pluginId);
        }
    }
}
