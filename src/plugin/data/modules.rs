use std::sync::Arc;
use std::collections::HashMap;

use plugin::api::plugin::*;

pub struct PluginManagerModulesData {
    modules: HashMap<Arc<str>, Box<PluginModule>>    
}

impl PluginManagerModulesData {
    pub fn new() -> PluginManagerModulesData {
        return PluginManagerModulesData{
            modules: HashMap::new()
        };
    }

    pub fn add_module(&mut self, module: Box<PluginModule>) {
        panic!("Not implemented yet");
    }
}
