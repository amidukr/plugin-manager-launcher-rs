use std::sync::Arc;

use plugin::api::module::PluginModule;

pub struct PluginId {
    module_name: Arc<String>,
    plugin_name: Arc<String>
}

pub enum PluginActionEnum {
    Start,
    Stop
}

pub struct PluginAction(pub PluginId, pub PluginActionEnum);

pub struct PluginConfiguration {
    plugins: Vec<PluginAction>
}

pub enum PluginStatusEnum{
    Active,
    Inactive
}

pub struct PluginMetaInformation {
    plugin_name: Arc<String>,
    short_description: Arc<String>,
    status: PluginStatusEnum
}

pub struct ModulesMetaInformation {
    plugings: Vec<PluginMetaInformation>,
    short_description: Arc<String>,
    library_file_path: Arc<String>
}

pub struct PluginConfigurationStatus {
    modules: Vec<ModulesMetaInformation>
}

pub trait PluginManager : Sync + Send {
    fn add_external_module(&self, library_file_path: &Arc<String>);
    fn add_module(&self, module: Box<PluginModule>);

    fn get_status(&self) -> Arc<PluginConfigurationStatus>;

    fn reload_configuration(&self, configuration: &PluginConfiguration) -> Result<(), &str>;
}


impl PluginConfiguration {
    pub fn new(plugins: Vec<PluginAction>) -> PluginConfiguration {
        return PluginConfiguration {
            plugins: plugins
        }
    }
}

impl PluginId {
    pub fn new(module_name: Arc<String>, plugin_name: Arc<String>) -> PluginId {
        return PluginId {
            module_name: module_name,
            plugin_name: plugin_name
        }
    }

    pub fn from_str(module_name: &str, plugin_name: &str) -> PluginId {
        return PluginId::new(
                Arc::new(module_name.to_owned()),
                Arc::new(plugin_name.to_owned())
                );
    }
}

impl PluginAction {
    pub fn from_str(module_name: &str, plugin_name: &str, action: PluginActionEnum) -> PluginAction {
        return PluginAction(PluginId::from_str(module_name, plugin_name), action);
    }
}