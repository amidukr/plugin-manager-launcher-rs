use std::sync::Arc;

use plugin::api::module::PluginModule;

pub struct PluginId {
    module_name: Arc<String>,
    plugin_name: Arc<String>
}

#[derive(PartialEq, Copy, Clone)]
pub enum PluginActionEnum {
    Start,
    Stop
}

pub struct PluginAction {
    plugin_id: PluginId, 
    action:  PluginActionEnum
}

pub struct PluginConfiguration {
    pub plugins: Vec<PluginAction>
}

#[derive(PartialEq, Copy, Clone)]
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
    pub fn new() -> PluginConfiguration {
        return PluginConfiguration {
            plugins: Vec::new()
        }
    }

    pub fn start_plugin<S: Into<String>>(mut self, module_name: S, plugin_name: S) -> Self {
        return self.add_plugin_action(PluginAction::new(module_name, plugin_name, PluginActionEnum::Start));
    }

    pub fn stop_plugin<S: Into<String>>(mut self, module_name: S, plugin_name: S) -> Self {
        return self.add_plugin_action(PluginAction::new(module_name, plugin_name, PluginActionEnum::Stop));
    }

    pub fn add_plugin_action(mut self, plguin_action: PluginAction) -> Self {
        self.plugins.push(plguin_action);
        return self;
    }
}

impl PluginAction {
    pub fn new<S: Into<String>>(module_name: S, plugin_name: S, action: PluginActionEnum) -> PluginAction {
        return PluginAction {
            plugin_id: PluginId::new(module_name, plugin_name), 
            action: action
        }
    }

    pub fn get_plugin_id(&self) -> &PluginId {
        return &self.plugin_id;
    }

    pub fn get_action(&self) -> PluginActionEnum {
        return self.action;
    }
}

impl PluginId {
    pub fn new<S: Into<String>>(module_name: S, plugin_name: S) -> PluginId {
        return PluginId {
            module_name: Arc::new(module_name.into()),
            plugin_name: Arc::new(plugin_name.into())
        }
    }
}
