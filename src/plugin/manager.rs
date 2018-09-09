use std::sync::Arc;
use std::sync::{RwLock, RwLockWriteGuard};

use plugin::container::*;

use plugin::api::container::*;
use plugin::api::manager::*;
use plugin::api::modules::*;

use plugin::data::modules::*;
use plugin::data::status::*;

pub struct PluginManagerData{
    pub modules_data: PluginManagerModulesData,
    pub status_data: PluginManagerStatusData
}

pub struct PluginManagerEngine
{
    locked_data: RwLock<PluginManagerData>,
    plugin_container: SharedPluginContainer
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
            locked_data: RwLock::new(PluginManagerData::new()),
            plugin_container: SharedPluginContainer::new() //TODO: renamed to components container
        });
    }

    pub fn write_lock(&self) -> RwLockWriteGuard<PluginManagerData> {
        return self.locked_data.write().unwrap();
    }

    pub fn get_plugin_container(&self) -> &PluginContainer {
        return &self.plugin_container;
    }
}

impl PluginManagerData {
    pub fn new() -> PluginManagerData {
        return PluginManagerData {
            modules_data: PluginManagerModulesData::new(),
            status_data: PluginManagerStatusData::new()
        }
    }
}
