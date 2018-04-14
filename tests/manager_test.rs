extern crate plugin_launcher;

use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::TryRecvError;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::collections::HashMap;

use plugin_launcher::plugin::api::container::PluginContainer;
use plugin_launcher::plugin::manager::PluginManagerFactory;
use plugin_launcher::plugin::helpers::module::*;
use plugin_launcher::plugin::api::module::*;
use plugin_launcher::plugin::api::manager::*;

use std::sync::Mutex;

struct MockPlugin {
    id: &'static str, 
    tx: Mutex<Sender<String>>
}

fn create_mock_plugin_module(tx: Sender<String>) -> Box<PluginModule>{
    /*return PluginModuleHelper::new("mock-module", "Mock module for unit testing")
        .add_plugin("mock-plugin-1", "Mock plugin 1", MockPlugin{id: "mock-plugin-1", tx: Mutex::new(tx.clone())}))
        .add_plugin("mock-plugin-1", "Mock plugin 1", MockPlugin{id: "mock-plugin-1", tx: Mutex::new(tx.clone())}));*/

    return PluginModuleHelper::from_str(
        "mock-module",
        "Mock module for unit testing",
        vec![
        PluginHelper::from_str("mock-plugin-1", "Mock plugin 1", MockPlugin{id: "mock-plugin-1", tx: Mutex::new(tx.clone())}), 
        PluginHelper::from_str("mock-plugin-2", "Mock plugin 2", MockPlugin{id: "mock-plugin-2", tx: Mutex::new(tx)})
        ]);
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
fn it_start_plugin_test() {
    let (tx, rx) = mpsc::channel();

    let plugin_manager: Arc<PluginManager> = PluginManagerFactory::new();
    let mock_plugin_module: Box<PluginModule> = create_mock_plugin_module(tx);

    plugin_manager.add_module(mock_plugin_module);

    plugin_manager.reload_configuration(&PluginConfiguration::new(
        vec![
            PluginAction::from_str("mock-module", "mock-plugin-1", PluginActionEnum::Start)
        ]
    )).unwrap();

    assert_eq!("mock-plugin-1: plugin register components", rx.recv().unwrap());
    assert_eq!("mock-plugin-1: plugin start", rx.recv().unwrap());
}

#[test]
fn it_start_two_plugins_test() {
    let (tx, rx) = mpsc::channel();

    let plugin_manager: Arc<PluginManager> = PluginManagerFactory::new();
    let mock_plugin_module: Box<PluginModule> = create_mock_plugin_module(tx);

    plugin_manager.add_module(mock_plugin_module);

    plugin_manager.reload_configuration(&PluginConfiguration::new(
        vec![
            PluginAction::from_str("mock-module", "mock-plugin-1", PluginActionEnum::Start),
            PluginAction::from_str("mock-module", "mock-plugin-2", PluginActionEnum::Start)
        ]
    )).unwrap();

    assert_eq!("mock-plugin-1: plugin register components", rx.recv().unwrap());
    assert_eq!("mock-plugin-2: plugin register components", rx.recv().unwrap());
    assert_eq!("mock-plugin-1: plugin start", rx.recv().unwrap());
    assert_eq!("mock-plugin-2: plugin start", rx.recv().unwrap());
}

#[test]
fn it_start_plugin_later_test() {
    let (tx, rx) = mpsc::channel();

    let plugin_manager: Arc<PluginManager> = PluginManagerFactory::new();
    let mock_plugin_module: Box<PluginModule> = create_mock_plugin_module(tx);

    plugin_manager.add_module(mock_plugin_module);

    plugin_manager.reload_configuration(&PluginConfiguration::new(
        vec![
            PluginAction::from_str("mock-module", "mock-plugin-1", PluginActionEnum::Start)
        ]
    )).unwrap();

    assert_eq!("mock-plugin-1: plugin register components", rx.recv().unwrap());
    assert_eq!("mock-plugin-1: plugin start", rx.recv().unwrap());

    plugin_manager.reload_configuration(&PluginConfiguration::new(
        vec![
            PluginAction::from_str("mock-module", "mock-plugin-2", PluginActionEnum::Start)
        ]
    )).unwrap();
    
    assert_eq!("mock-plugin-2: plugin register components", rx.recv().unwrap());
    assert_eq!("mock-plugin-2: plugin start", rx.recv().unwrap());
}

#[test]
fn it_reload_plugin_test() {
    let (tx, rx) = mpsc::channel();

    let plugin_manager: Arc<PluginManager> = PluginManagerFactory::new();
    let mock_plugin_module: Box<PluginModule> = create_mock_plugin_module(tx);

    plugin_manager.add_module(mock_plugin_module);

    plugin_manager.reload_configuration(&PluginConfiguration::new(
        vec![
            PluginAction::from_str("mock-module", "mock-plugin-1", PluginActionEnum::Start)
        ]
    )).unwrap();

    assert_eq!("mock-plugin-1: plugin register components", rx.recv().unwrap());
    assert_eq!("mock-plugin-1: plugin start", rx.recv().unwrap());

    plugin_manager.reload_configuration(&PluginConfiguration::new(
        vec![
            PluginAction::from_str("mock-module", "mock-plugin-1", PluginActionEnum::Stop),
            PluginAction::from_str("mock-module", "mock-plugin-2", PluginActionEnum::Start)
        ]
    )).unwrap();
    
    assert_eq!("mock-plugin-1: plugin stop", rx.recv().unwrap());
    assert_eq!("mock-plugin-2: plugin register components", rx.recv().unwrap());
    assert_eq!("mock-plugin-2: plugin start", rx.recv().unwrap());
}

#[test]
fn it_fail_test() {
    let (tx, rx) = mpsc::channel();

    let plugin_manager: Arc<PluginManager> = PluginManagerFactory::new();
    let mock_plugin_module: Box<PluginModule> = create_mock_plugin_module(tx);

    plugin_manager.add_module(mock_plugin_module);

    let result = plugin_manager.reload_configuration(&PluginConfiguration::new(
        vec![
            PluginAction::from_str("mock-module", "mock-plugin-1", PluginActionEnum::Start),
            PluginAction::from_str("wrong-module-name", "mock-plugin-1", PluginActionEnum::Start)
        ]
    ));

    assert_eq!(Err("Module 'wrong-module-name' not found"), result);
    assert_eq!(Err(TryRecvError::Empty), rx.try_recv());
}
