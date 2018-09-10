use std::sync::Arc;
use std::sync::{RwLock, RwLockWriteGuard};

use plugin::container::*;

use plugin::api::container::*;
use plugin::api::manager::*;
use plugin::api::modules::*;

use plugin::data::modules::*;
use plugin::data::status::*;

pub struct PluginManagerEngine
{
    modules_data: RwLock<PluginManagerModulesData>,
    status_data:  RwLock<PluginManagerStatusData>,

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
            modules_data: RwLock::new(PluginManagerModulesData::new()),
            status_data:  RwLock::new(PluginManagerStatusData::new()),
            plugin_container: SharedPluginContainer::new() //TODO: renamed to components container
        });
    }

    pub fn modules_write_lock(&self) -> RwLockWriteGuard<PluginManagerModulesData> {
        return self.modules_data.write().unwrap();
    }

    pub fn status_write_lock(&self) -> RwLockWriteGuard<PluginManagerStatusData> {
        return self.status_data.write().unwrap();
    }

    pub fn get_plugin_container(&self) -> &PluginContainer {
        return &self.plugin_container;
    }
}
