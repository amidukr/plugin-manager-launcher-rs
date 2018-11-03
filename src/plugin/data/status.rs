use plugin::api::plugin::*;
use plugin::api::modules::*;

use std::collections::HashSet;

pub struct PluginManagerStatusData {
    active_plugins: HashSet<PluginId>
}


impl PluginManagerStatusData {
    pub fn new() -> PluginManagerStatusData {
        return PluginManagerStatusData {
            active_plugins : HashSet::new()
        }
    }
    pub fn set_plugin_status(&mut self, plugin_id: &PluginId, status: PluginStatusEnum)
    {
        match status 
        {
            PluginStatusEnum::Active =>   self.active_plugins.insert(plugin_id.clone()),
            PluginStatusEnum::Inactive => self.active_plugins.remove(plugin_id)
        };
    }

    pub fn get_plugin_status(&self, plugin_id: &PluginId) -> PluginStatusEnum {
        if(self.active_plugins.contains(plugin_id)) 
        {
            return PluginStatusEnum::Active
        }
        else
        {
            return PluginStatusEnum::Inactive
        }
    }
}
