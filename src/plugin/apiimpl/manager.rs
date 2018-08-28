// use std::sync::Arc;
// use std::sync::RwLock;
// use std::collections::HashMap;

// use plugin::container::SharedPluginContainer;
// use plugin::api::manager::*;
// use plugin::api::module::*;


// struct PluginManagerModulesApi {
//     engine: Arc<PluginManagerEngine>
// }

// impl PluginManagerModules for PluginManagerModulesApi {

// }

// fn main() {
//     let plugin_manager: Arc<PluginManager> = PluginManagerEngine::new();
    
//     let plugin_modules = plugin_manager.get_plugin_modules();

//     plugin_modules.add_module(...);
// }


// pub struct PluginManagerFactory;
// pub struct PluginManagerUtils;

// struct PluginStatusInfo<'a> {
//     plugin: &'a mut Plugin,
//     plugin_status: PluginStatusEnum
// }

// struct PluginModulesContainer {
//     modules: HashMap<Arc<str>, Box<PluginModule>>    
// }

// struct PluginManagerHelper {
//     modules_lock: RwLock<PluginModulesContainer>,
//     plugin_container: SharedPluginContainer
// }

// impl PluginManagerUtils {
//     pub fn get_plugins_for_configuration<'a, 'b>(plugin_container: &'a mut PluginModulesContainer, 
//                                      configuration: &'b PluginConfiguration, 
//                                      action: PluginActionEnum) -> Vec<&'a mut Plugin>{
//         let plugin_ids = configuration.get_plugin_ids(action);
//         return modules.get_plugins_by_ids(plugin_ids);
//     }
// }

// impl PluginManager for PluginManagerHelper {
//     fn add_external_module(&self, library_file_path: &Arc<str>) {
//         panic!("Operation Unsupported yet");
//     }

//     fn add_module(&self, module: Box<PluginModule>) {
//         let modules = &mut *self.modules_lock.write().unwrap();

//         modules.add_module(module);
//     }

//     fn get_status(&self) -> Arc<PluginConfigurationStatus> {
//         panic!("Operation Unsupported yet");
//     }

//     fn apply_configuration(&self, configuration: &PluginConfiguration) -> Result<(), &str> {
//         let modules = &mut *self.modules_lock.write().unwrap();

//         let start_plugin_ids = configuration.get_plugin_ids(PluginActionEnum::Start);
//         let start_plugins = modules.get_plugins_by_ids(start_plugin_ids);

//         {
//             let plugins_to_stop = PluginManagerUtils::get_plugins_for_configuration(modules, configuration, PluginActionEnum::Stop);

//             for plugin_to_stop in plugins_to_stop {
//                 plugin_to_stop.stop_plugin();
                
//                 modules.set_plugin_status(plugin_to_stop, PluginStatusEnum::Active)
//             }
//         }

//         {
//             let plugins_to_start = PluginManagerUtils::get_plugins_for_configuration(modules, configuration, PluginActionEnum::Start);

//             for plugin_to_start in plugins_to_start {
//                 plugin_to_start.register_components(&self.plugin_container);
//                 plugin_to_start.start_plugin();

//                 modules.set_plugin_status(plugin_to_start, PluginStatusEnum::Active)
//             }
//         }
//     }
// }

// impl PluginModulesContainer {
//     fn new() -> PluginModulesContainer {
//         return PluginModulesContainer{
//             modules: HashMap::new()
//         };
//     }

//     fn add_module(&mut self, plugin_module: Box<PluginModule>) {
//         self.modules.insert(plugin_module.get_module_name().clone(), plugin_module);
//     }

//     fn get_plugins(&mut self) -> Vec<&mut Plugin>{
//         panic!("Operation unsupported yet");
//     }

//     fn set_plugin_status(&mut self, plugin: &Plugin, status: PluginStatusEnum) {
//         panic!("Operation unsupported yet");
//     }
// }



// impl PluginManagerFactory {
//     pub fn new() -> Arc<PluginManager> {
//         return Arc::new(PluginManagerHelper{
//             modules_lock: RwLock::new(PluginModulesContainer::new()),
//             plugin_container: SharedPluginContainer::new()
//         });
//     }
// }
