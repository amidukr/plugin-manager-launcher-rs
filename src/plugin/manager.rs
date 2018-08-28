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
    modules_data: PluginManagerModulesData
}

impl PluginManager for Arc<PluginManagerEngine>
{
    fn get_plugin_manager(&self) -> Arc<PluginManager>
    {
        return Arc::new(self.clone());
    }

    fn get_plugin_modules(&self) -> Arc<PluginManagerModules> 
    {
        return Arc::new(self.clone());
    }
}

impl PluginManagerEngine 
{
    pub fn new() -> Arc<PluginManager>
    {
        return PluginManagerEngine::new_engine().get_plugin_manager();
    }

    fn new_engine() -> Arc<PluginManagerEngine>
    {
        return Arc::new(PluginManagerEngine {
            modules_data: PluginManagerModulesData::new()
        });
    }
}
