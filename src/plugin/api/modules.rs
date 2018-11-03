use std::sync::Arc;
use std::collections::HashSet;

use plugin::api::error::PluginManagerError;
use plugin::api::error::ErrorLog;
use plugin::api::plugin::PluginModule;

use plugin::helpers::langutils::new_str;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct PluginId {
    module_name: Arc<str>,
    plugin_name: Arc<str>
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

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum PluginStatusEnum{
    Active,
    Inactive
}

pub struct PluginMetaInformation {
    plugin_name: Arc<str>,
    short_description: Arc<str>,
    status: PluginStatusEnum
}

pub struct ModulesMetaInformation {
    plugings: Vec<PluginMetaInformation>,
    short_description: Arc<str>,
    library_file_path: Arc<str>
}

pub struct PluginConfigurationStatus {
    modules: Vec<ModulesMetaInformation>
}

pub trait PluginManagerModules : Sync + Send {
    fn add_external_module(&self, library_file_path: &Arc<str>);
    fn add_module(&self, module: Box<PluginModule>);

    fn get_status(&self) -> Arc<PluginConfigurationStatus>;

    fn apply_configuration(&self, configuration: &PluginConfiguration) -> ErrorLog;
}


impl PluginConfiguration {
    pub fn new() -> PluginConfiguration {
        return PluginConfiguration {
            plugins: Vec::new()
        }
    }

    pub fn start_plugin<S: Into<Arc<str>>>(mut self, module_name: S, plugin_name: S) -> Self {
        return self.add_plugin_action(PluginAction::new(module_name, plugin_name, PluginActionEnum::Start));
    }

    pub fn stop_plugin<S: Into<Arc<str>>>(mut self, module_name: S, plugin_name: S) -> Self {
        return self.add_plugin_action(PluginAction::new(module_name, plugin_name, PluginActionEnum::Stop));
    }

    pub fn add_plugin_action(mut self, plguin_action: PluginAction) -> Self {
        self.plugins.push(plguin_action);
        return self;
    }

    fn get_modules(&self, action: PluginActionEnum) -> Vec<Arc<str>> {
        let mut result:HashSet<_> = self.plugins.iter()
           .filter(|x| x.action == action)
           .map(|x| x.plugin_id.module_name.clone() )
           .collect();

        return result.into_iter().collect();
    }

    pub fn get_plugin_actions(&self) -> &Vec<PluginAction>
    {
        return &self.plugins;
    }

    pub fn get_list_of_plugin_id_for_action(&self, action_enum: PluginActionEnum) -> Vec<&PluginId>
    {
        return self.plugins.iter()
        .filter(|x| x.action == action_enum)
        .map(|x| x.get_plugin_id())
        .collect();
    }

    fn get_module_plugins<S: Into<Arc<str>>>(&self, action: PluginActionEnum, module_name: S) -> Vec<Arc<str>> {
        let module_name_str = module_name.into();
        let mut result:HashSet<_> = self.plugins.iter()
           .filter(|x| x.action == action && x.plugin_id.module_name == module_name_str )
           .map(|x| x.plugin_id.plugin_name.clone() )
           .collect();

        return result.into_iter().collect();
    }

    pub fn get_start_modules(&self) -> Vec<Arc<str>> {
        self.get_modules(PluginActionEnum::Start)
    }

    pub fn get_stop_modules(&self) -> Vec<Arc<str>> {
        self.get_modules(PluginActionEnum::Stop)
    }

    pub fn get_start_plugins<S: Into<Arc<str>>>(&self, module_name: S) -> Vec<Arc<str>> {
        self.get_module_plugins(PluginActionEnum::Start, module_name)
    }

    pub fn get_stop_plugins<S: Into<Arc<str>>>(&self, module_name: S) -> Vec<Arc<str>> {
        self.get_module_plugins(PluginActionEnum::Stop, module_name)
    }
}

impl PluginAction {
    pub fn new<S: Into<Arc<str>>>(module_name: S, plugin_name: S, action: PluginActionEnum) -> PluginAction {
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
    pub fn new<S: Into<Arc<str>>>(module_name: S, plugin_name: S) -> PluginId {
        return PluginId {
            module_name: module_name.into(),
            plugin_name: plugin_name.into()
        }
    }

    pub fn get_module_name(&self) -> &Arc<str>
    {
        return &self.module_name;
    }

    pub fn get_plugin_name(&self) -> &Arc<str>
    {
        return &self.plugin_name;
    }
}
