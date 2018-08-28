extern crate plugin_launcher;

use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::TryRecvError;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::collections::HashMap;

use plugin_launcher::plugin::manager::PluginManagerEngine;

use plugin_launcher::plugin::api::container::PluginContainer;
use plugin_launcher::plugin::api::plugin::*;
use plugin_launcher::plugin::api::modules::*;
use plugin_launcher::plugin::api::manager::*;

use plugin_launcher::plugin::helpers::plugin::*;

use plugin_launcher::plugin::apiimpl::manager::*;

use plugin_launcher::plugin::helpers::langutils::{vec_of_str, sort_and_return, new_str};

use std::sync::Mutex;

struct MockPlugin {
    id: &'static str, 
    tx: Mutex<Sender<String>>
}

fn create_mock_plugin_module(tx: Sender<String>) -> Box<PluginModule>{
    return PluginModuleHelper::new("mock-module", "Mock module for unit testing")
        .add_plugin("mock-plugin-1", "Mock plugin 1", MockPlugin{id: "mock-plugin-1", tx: Mutex::new(tx.clone())})
        .add_plugin("mock-plugin-1", "Mock plugin 1", MockPlugin{id: "mock-plugin-1", tx: Mutex::new(tx.clone())})
        .create();
}

impl PluginEventsHandler for MockPlugin {

    fn register_components(&mut self, plugin_container: &PluginContainer) {
        self.tx.lock().unwrap().send(format!("{}: plugin register components", self.id)).unwrap();
    }

    fn start_plugin(&mut self) {
        self.tx.lock().unwrap().send(format!("{}: plugin start", self.id)).unwrap();
    }

    fn stop_plugin(&mut self) {
        self.tx.lock().unwrap().send(format!("{}: plugin stop", self.id)).unwrap();
    }
}


#[test]
fn it_plugin_configuration_test() {
    let plugin_configuration = PluginConfiguration::new()
                                    .start_plugin("mock-module1", "mock-plugin-1")
                                    .stop_plugin ("mock-module1", "mock-plugin-2")

                                    .start_plugin("mock-module2", "mock-plugin-3")
                                    .stop_plugin ("mock-module2", "mock-plugin-4")
                                    .start_plugin("mock-module2", "mock-plugin-5")

                                    .stop_plugin ("mock-module3", "mock-plugin-6");

    assert_eq!(vec_of_str(&["mock-module1", "mock-module2"]),                 sort_and_return(plugin_configuration.get_start_modules()));
    assert_eq!(vec_of_str(&["mock-module1", "mock-module2", "mock-module3"]), sort_and_return(plugin_configuration.get_stop_modules()));

    assert_eq!(vec_of_str(&["mock-plugin-1"]),                  sort_and_return(plugin_configuration.get_start_plugins("mock-module1")));
    assert_eq!(vec_of_str(&["mock-plugin-3", "mock-plugin-5"]), sort_and_return(plugin_configuration.get_start_plugins("mock-module2")));
    assert_eq!(vec_of_str(&[]),                                 sort_and_return(plugin_configuration.get_start_plugins("mock-module3")));

    assert_eq!(vec_of_str(&["mock-plugin-2"]), sort_and_return(plugin_configuration.get_stop_plugins("mock-module1")));
    assert_eq!(vec_of_str(&["mock-plugin-4"]), sort_and_return(plugin_configuration.get_stop_plugins("mock-module2")));
    assert_eq!(vec_of_str(&["mock-plugin-6"]), sort_and_return(plugin_configuration.get_stop_plugins("mock-module3")));
}

#[test]
fn it_start_plugin_test() {
    let (tx, rx) = mpsc::channel();

    let plugin_manager = PluginManagerEngine::new(); 
    let plugin_module = plugin_manager.get_plugin_modules();

    let mock_plugin_module: Box<PluginModule> = create_mock_plugin_module(tx);

    plugin_module.add_module(mock_plugin_module);

    let plugin_configuration = PluginConfiguration::new()
                                    .start_plugin("mock-module", "mock-plugin-1");

    plugin_module.apply_configuration(&plugin_configuration).expect("Can't reload plugin configuration");

    assert_eq!("mock-plugin-1: plugin register components", rx.recv().unwrap());
    assert_eq!("mock-plugin-1: plugin start", rx.recv().unwrap());
}

#[test]
fn it_start_two_plugins_test() {
    let (tx, rx) = mpsc::channel();

    let plugin_manager = PluginManagerEngine::new(); 
    let plugin_module = plugin_manager.get_plugin_modules();

    plugin_module.add_module(create_mock_plugin_module(tx));

    let plugin_configuration = PluginConfiguration::new()
                                    .start_plugin("mock-module", "mock-plugin-1")
                                    .start_plugin("mock-module", "mock-plugin-2");

    plugin_module.apply_configuration(&plugin_configuration).expect("Can't reload plugin configuration");

    assert_eq!("mock-plugin-1: plugin register components", rx.recv().unwrap());
    assert_eq!("mock-plugin-2: plugin register components", rx.recv().unwrap());
    assert_eq!("mock-plugin-1: plugin start", rx.recv().unwrap());
    assert_eq!("mock-plugin-2: plugin start", rx.recv().unwrap());
}

#[test]
fn it_start_plugin_later_test() {
    let (tx, rx) = mpsc::channel();

    let plugin_manager = PluginManagerEngine::new(); 
    let plugin_module = plugin_manager.get_plugin_modules();

    let mock_plugin_module: Box<PluginModule> = create_mock_plugin_module(tx);

    plugin_module.add_module(mock_plugin_module);

    let plugin_configuration = PluginConfiguration::new()
                                    .start_plugin("mock-module", "mock-plugin-1");

    plugin_module.apply_configuration(&plugin_configuration).expect("Can't reload plugin configuration");

    assert_eq!("mock-plugin-1: plugin register components", rx.recv().unwrap());
    assert_eq!("mock-plugin-1: plugin start", rx.recv().unwrap());

    let plugin_configuration = PluginConfiguration::new()
                                    .start_plugin("mock-module", "mock-plugin-2");

    plugin_module.apply_configuration(&plugin_configuration).expect("Can't reload plugin configuration");
    
    assert_eq!("mock-plugin-2: plugin register components", rx.recv().unwrap());
    assert_eq!("mock-plugin-2: plugin start", rx.recv().unwrap());
}

#[test]
fn it_reload_plugin_test() {
    let (tx, rx) = mpsc::channel();

    let plugin_manager = PluginManagerEngine::new(); 
    let plugin_module = plugin_manager.get_plugin_modules();

    let mock_plugin_module: Box<PluginModule> = create_mock_plugin_module(tx);

    plugin_module.add_module(mock_plugin_module);

    let plugin_configuration = PluginConfiguration::new()
                                    .start_plugin("mock-module", "mock-plugin-1");

    plugin_module.apply_configuration(&plugin_configuration).expect("Can't reload plugin configuration");

    assert_eq!("mock-plugin-1: plugin register components", rx.recv().unwrap());
    assert_eq!("mock-plugin-1: plugin start", rx.recv().unwrap());

    let plugin_configuration = PluginConfiguration::new()
                                    .stop_plugin("mock-module", "mock-plugin-1")
                                    .start_plugin("mock-module", "mock-plugin-2");

    plugin_module.apply_configuration(&plugin_configuration).expect("Can't reload plugin configuration");
    
    assert_eq!("mock-plugin-1: plugin stop", rx.recv().unwrap());
    assert_eq!("mock-plugin-2: plugin register components", rx.recv().unwrap());
    assert_eq!("mock-plugin-2: plugin start", rx.recv().unwrap());
}

#[test]
fn it_fail_test() {
    let (tx, rx) = mpsc::channel();

    let plugin_manager = PluginManagerEngine::new(); 
    let plugin_module = plugin_manager.get_plugin_modules();

    let mock_plugin_module: Box<PluginModule> = create_mock_plugin_module(tx);

    plugin_module.add_module(mock_plugin_module);

    let plugin_configuration = PluginConfiguration::new()
                                    .stop_plugin("mock-module", "mock-plugin-1")
                                    .start_plugin("wrong-module-name", "mock-plugin-2");

    let result = plugin_module.apply_configuration(&plugin_configuration);

    assert_eq!(Err("Module 'wrong-module-name' not found"), result);
    assert_eq!(Err(TryRecvError::Empty), rx.try_recv());
}

#[test]
fn it_unsupported_operations_not_unit_tested_yet() {
    let plugin_manager = PluginManagerEngine::new(); 
    let plugin_module = plugin_manager.get_plugin_modules();

    plugin_module.add_external_module(&new_str("test"));
    plugin_module.get_status();
}
