use plugin::api::plugin::*;
use plugin::api::modules::*;

pub struct PluginManagerStatusData {
}


impl PluginManagerStatusData {
    pub fn new() -> PluginManagerStatusData {
        return PluginManagerStatusData {}
    }
    pub fn set_plugin_status(&mut self, plugin: &PluginId, status: PluginStatusEnum) {
        panic!("Not supported yet");
    }

    pub fn get_plugin_status(&mut self, plugin: &PluginId) -> PluginStatusEnum {
        panic!("Not supported yet");
    }
}
