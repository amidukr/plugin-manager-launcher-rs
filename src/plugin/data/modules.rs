use std::sync::Arc;
use std::collections::HashMap;
use std::sync::RwLock;

use plugin::api::plugin::*;

pub struct PluginModulesContainer {
    modules: HashMap<Arc<str>, Box<PluginModule>>    
}

pub struct PluginManagerModulesData {
    lock: RwLock<PluginModulesContainer>
}

impl PluginModulesContainer {
    fn new() -> PluginModulesContainer {
        return PluginModulesContainer{
            modules: HashMap::new()
        };
    }
}

impl PluginManagerModulesData {
    pub fn new() -> PluginManagerModulesData {
        return PluginManagerModulesData {
            lock: RwLock::new(PluginModulesContainer::new()),
        };
    }
}
