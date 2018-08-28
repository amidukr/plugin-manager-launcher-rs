use std::sync::Arc;
use std::sync::RwLock;
use std::collections::HashMap;

use plugin::api::manager::*;
use plugin::api::modules::*;

use plugin::helpers::plugin::*;

use plugin::apiimpl::manager::*;
use plugin::apiimpl::modules::*;

use plugin::data::modules::*;

pub struct PluginManagerEngine
{
    plugin_module: PluginManagerModulesApi,

    modules_data: PluginManagerModulesData
}

impl PluginManager for PluginManagerEngine {
    fn get_plugin_modules(&self) -> &PluginManagerModules {
        return &self.plugin_module;
    }
}

impl PluginManagerEngine 
{
    pub fn new() -> Arc<PluginManager> 
    {
        return Arc::new(PluginManagerEngine {
            plugin_module: PluginManagerModulesApi::new(),

            modules_data: PluginManagerModulesData::new()
        });
    }
}
