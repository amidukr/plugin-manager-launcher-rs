use std::sync::Arc;
use std::collections::HashMap;


use plugin::api::module::*;
use plugin::api::container::*;

pub trait PluginEventsHandler {
    fn register_components(&mut self, plugin_container: &PluginContainer);
    fn start_plugin(&mut self);
    fn stop_plugin(&mut self);
}

pub struct PluginHelper<T: PluginEventsHandler> {
    plugin_name: Arc<str>,
    plugin_short_description: Arc<str>,
    events_handler: T
}    

pub struct PluginModuleHelper {
    module_name: Arc<str>,
    module_short_description: Arc<str>,
    plugin_names: Arc<Vec<Arc<str>>>,
    plugin_map: HashMap<Arc<str>, Box<Plugin>>
}

impl <T: PluginEventsHandler + Sync + Send> Plugin for PluginHelper<T> {
    fn get_plugin_name(&self) -> &Arc<str> {
        return &self.plugin_name;
    }

    fn get_plugin_short_description(&self) -> &Arc<str> {
        return &self.plugin_short_description;
    }

    
    fn register_components(&mut self, plugin_container: &PluginContainer) {
        self.events_handler.register_components(plugin_container);
    }

    fn start_plugin(&mut self) {
        self.events_handler.start_plugin();
    }

    fn stop_plugin(&mut self) {
        self.events_handler.stop_plugin();
    }
}

impl <T: PluginEventsHandler + Sync + Send + 'static>  PluginHelper<T> {
    pub fn new<S: Into<Arc<str>>>( plugin_name: S, 
                plugin_short_description: S,
                events_handler: T) -> Box<Plugin> {

        return Box::new(PluginHelper{
            plugin_name: plugin_name.into(),
            plugin_short_description: plugin_short_description.into(),
            events_handler: events_handler
        });
    }
}

impl PluginModule for PluginModuleHelper {
    fn get_module_name(&self) -> &Arc<str> {
        return &self.module_name;
    }

    fn get_module_short_description(&self) -> &Arc<str> {
        return &self.module_short_description;
    }

    fn get_plugins_names(&self) -> Arc<Vec<Arc<str>>> {
        return self.plugin_names.clone();
    }

    fn get_plugin(&mut self, plugin_name: & Arc<str>) -> Option<&mut Plugin> {
        return match self.plugin_map.get_mut(plugin_name) {
            Some(x) => Some(x.as_mut()),
            None => None,
        };
    }
}

impl PluginModuleHelper {
    pub fn new<S: Into<Arc<str>>>(module_name: S, 
               module_short_description: S) -> PluginModuleHelper{
        return PluginModuleHelper{
            module_name: module_name.into(),
            module_short_description: module_short_description.into(),
            plugin_names: Arc::new(Vec::new()),
            plugin_map: HashMap::new()
        };
    }

    pub fn add_plugin<T: PluginEventsHandler + Sync + Send + 'static, S: Into<Arc<str>>>(mut self, 
                    plugin_name: S, 
                    plugin_short_description: S,
                    events_handler: T) -> Self{
        return self.put_plugin(PluginHelper::new(plugin_name, plugin_short_description, events_handler));
    }

    pub fn put_plugin(mut self, plugin: Box<Plugin>) -> Self{

        Arc::make_mut(&mut self.plugin_names).push(plugin.get_plugin_name().clone());
        self.plugin_map.insert(plugin.get_plugin_name().clone(), plugin);

        return self;
    }

    pub fn create(self) -> Box<PluginModule>{
        return Box::new(self);
    }
}