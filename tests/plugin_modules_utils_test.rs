extern crate plugin_launcher;

use std::result::Result;
use std::sync::Arc;

use plugin_launcher::plugin::utils::modules::PluginModulesUtils;

use plugin_launcher::plugin::helpers::plugin::*;
use plugin_launcher::plugin::helpers::langutils::*;

use plugin_launcher::plugin::data::modules::*;
use plugin_launcher::plugin::data::status::*;


use plugin_launcher::plugin::api::error::PluginManagerError;
use plugin_launcher::plugin::api::modules::*;
use plugin_launcher::plugin::api::plugin::*;
use plugin_launcher::plugin::api::container::*;

use plugin_launcher::plugin::api::error::*;


struct MockPlugin;

impl PluginEventsHandler for MockPlugin {

    fn register_components(&mut self, plugin_container: &PluginContainer) {
        panic!("Not supported");
    }

    fn unregister_components(&mut self, plugin_container: &PluginContainer) {
        panic!("Not supported");
    }

    fn start_plugin(&mut self) {
        panic!("Not supported");
    }

    fn stop_plugin(&mut self) {
        panic!("Not supported");
    }
}

fn create_modules_data() -> PluginManagerModulesData {
    let mut modules = PluginManagerModulesData::new();
    
    modules.add_module(PluginModuleHelper::new("mock-module-1", "Mock module for unit testing")
                        .add_plugin("mock-plugin-1", "Mock plugin 1", MockPlugin)
                        .add_plugin("mock-plugin-2", "Mock plugin 2", MockPlugin)
                        .create());

    modules.add_module(PluginModuleHelper::new("mock-module-2", "Mock module for unit testing")
                        .add_plugin("mock-plugin-3", "Mock plugin 3", MockPlugin)
                        .add_plugin("mock-plugin-4", "Mock plugin 4", MockPlugin)
                        .add_plugin("mock-plugin-5", "Mock plugin 5", MockPlugin)
                        .create());

    modules.add_module(PluginModuleHelper::new("mock-module-3", "Mock module for unit testing")
                        .add_plugin("mock-plugin-6", "Mock plugin 6", MockPlugin)
                        .add_plugin("mock-plugin-7", "Mock plugin 7", MockPlugin)
                        .create());
    
    return modules;
}

#[test]
fn it_validate_success_test() {
    let mut modules = create_modules_data();
    let mut erorr_log = ErrorLog::new();

    let plugin_configuration = PluginConfiguration::new()
                                    .start_plugin("mock-module-1", "mock-plugin-1")
                                    .stop_plugin("mock-module-1",  "mock-plugin-1")
                                    .stop_plugin("mock-module-1",  "mock-plugin-2")
                                    .stop_plugin("mock-module-2",  "mock-plugin-3");

    PluginModulesUtils::validate_configuration(&mut modules, &plugin_configuration, &mut erorr_log);

    assert_eq!(&Vec::<PluginManagerError>::new(), erorr_log.get_errors());
    assert_eq!(true, erorr_log.is_no_alerts());
}

#[test]
fn it_validate_no_module_test() {
    let mut modules = create_modules_data();
    let mut erorr_log = ErrorLog::new();

    let plugin_configuration = PluginConfiguration::new()
                                    .start_plugin("bad-mock-module-1", "mock-plugin-1")
                                    .stop_plugin("bad-mock-module-2", "mock-plugin-1")                                    
                                    .stop_plugin("mock-module-2", "mock-plugin-3")
                                    .stop_plugin("bad-mock-module-2", "mock-plugin-2");

    PluginModulesUtils::validate_configuration(&mut modules, &plugin_configuration, &mut erorr_log);

    assert_eq!(vec![PluginManagerError::ModuleNotFound(new_str("bad-mock-module-1")),
                    PluginManagerError::ModuleNotFound(new_str("bad-mock-module-2"))], *erorr_log.get_errors());

    assert_eq!(true, erorr_log.is_failed());
}

#[test]
fn it_validate_no_plugin_test() {
    let mut modules = create_modules_data();
    let mut erorr_log = ErrorLog::new();

    let plugin_configuration = PluginConfiguration::new()
                                    .start_plugin("mock-module-1", "bad-mock-plugin-1")
                                    .stop_plugin("mock-module-1", "bad-mock-plugin-1")
                                    .stop_plugin("mock-module-1", "mock-plugin-2")
                                    .stop_plugin("mock-module-2", "bad-mock-plugin-3");

    PluginModulesUtils::validate_configuration(&mut modules, &plugin_configuration, &mut erorr_log);

    assert_eq!(vec![PluginManagerError::PluginNotFound(PluginId::new("mock-module-1", "bad-mock-plugin-1")),
                    PluginManagerError::PluginNotFound(PluginId::new("mock-module-2", "bad-mock-plugin-3"))], *erorr_log.get_errors());

    assert_eq!(true, erorr_log.is_failed());
}

#[test]
fn it_validate_no_plugins_and_some_modules_test() {
    let mut modules = create_modules_data();
    let mut erorr_log = ErrorLog::new();

    let plugin_configuration = PluginConfiguration::new()
                                    .start_plugin("mock-module-1", "bad-mock-plugin-1")
                                    .stop_plugin("mock-module-1", "bad-mock-plugin-1")
                                    .stop_plugin("bad-mock-module-1", "mock-plugin-2")
                                    .stop_plugin("mock-module-2", "bad-mock-plugin-3");

    PluginModulesUtils::validate_configuration(&mut modules, &plugin_configuration, &mut erorr_log);

    assert_eq!(vec![PluginManagerError::PluginNotFound(PluginId::new("mock-module-1", "bad-mock-plugin-1")),
                    PluginManagerError::ModuleNotFound(new_str("bad-mock-module-1")),
                    PluginManagerError::PluginNotFound(PluginId::new("mock-module-2", "bad-mock-plugin-3"))], *erorr_log.get_errors());

    assert_eq!(true, erorr_log.is_failed());
}

#[test]
fn it_get_plugins_for_start_stop() {
    let mut modules = create_modules_data();
    let mut status = PluginManagerStatusData::new();
    let mut erorr_log = ErrorLog::new();

    let plugin_configuration = PluginConfiguration::new()
                                    .stop_plugin("mock-module-1", "mock-plugin-1")
                                    .stop_plugin("mock-module-1", "mock-plugin-2")
                                    .stop_plugin("mock-module-2", "mock-plugin-3")
                                    .stop_plugin("mock-module-2", "mock-plugin-4")
                                    .start_plugin("mock-module-1", "mock-plugin-1")
                                    .start_plugin("mock-module-1", "mock-plugin-2")
                                    .start_plugin("mock-module-2", "mock-plugin-3")
                                    .start_plugin("mock-module-2", "mock-plugin-5")
                                    .start_plugin("mock-module-3", "mock-plugin-6");
    
    {
        let start_list:Vec<(&PluginId, &Arc<str>)> = PluginModulesUtils::validate_and_get_plugins_for_action(
                                                                    &mut modules, 
                                                                    &status, 
                                                                    &plugin_configuration, 
                                                                    PluginActionEnum::Start, 
                                                                    &mut erorr_log)
                                                                .into_iter()
                                                                .map(|(plugin_id, plugin)| (plugin_id, plugin.get_plugin_name()))
                                                                .collect();

        assert_eq!(vec![(&PluginId::new("mock-module-1", "mock-plugin-1"), &new_str("mock-plugin-1")),
                        (&PluginId::new("mock-module-1", "mock-plugin-2"), &new_str("mock-plugin-2")),
                        (&PluginId::new("mock-module-2", "mock-plugin-3"), &new_str("mock-plugin-3")),
                        (&PluginId::new("mock-module-2", "mock-plugin-5"), &new_str("mock-plugin-5")),
                        (&PluginId::new("mock-module-3", "mock-plugin-6"), &new_str("mock-plugin-6"))
        ], start_list);      

        assert_eq!(true, erorr_log.is_no_alerts());
    }
    
    {
        status.set_plugin_status(&PluginId::new("mock-module-1", "mock-plugin-1"), PluginStatusEnum::Active);
        status.set_plugin_status(&PluginId::new("mock-module-1", "mock-plugin-2"), PluginStatusEnum::Active);
        status.set_plugin_status(&PluginId::new("mock-module-2", "mock-plugin-3"), PluginStatusEnum::Active);
        status.set_plugin_status(&PluginId::new("mock-module-2", "mock-plugin-4"), PluginStatusEnum::Active);
        
        let stop_list:Vec<(&PluginId, &Arc<str>)> = PluginModulesUtils::validate_and_get_plugins_for_action(
                                                                    &mut modules, 
                                                                    &status, 
                                                                    &plugin_configuration, 
                                                                    PluginActionEnum::Stop, 
                                                                    &mut erorr_log)
                                                                .into_iter()
                                                                .map(|(plugin_id, plugin)| (plugin_id, plugin.get_plugin_name()))
                                                                .collect();
                                                                
        assert_eq!(vec![(&PluginId::new("mock-module-1", "mock-plugin-1"), &new_str("mock-plugin-1")),
                        (&PluginId::new("mock-module-1", "mock-plugin-2"), &new_str("mock-plugin-2")),
                        (&PluginId::new("mock-module-2", "mock-plugin-3"), &new_str("mock-plugin-3")),
                        (&PluginId::new("mock-module-2", "mock-plugin-4"), &new_str("mock-plugin-4"))
        ], stop_list);

        assert_eq!(true, erorr_log.is_no_alerts());
    }
}


#[test]
fn it_get_plugins_start_warnings() {
    let mut modules = create_modules_data();
    let mut status = PluginManagerStatusData::new();
    let mut erorr_log = ErrorLog::new();

    let plugin_configuration = PluginConfiguration::new()
                                    .stop_plugin("mock-module-1", "mock-plugin-1")
                                    .stop_plugin("mock-module-1", "mock-plugin-2")
                                    .stop_plugin("mock-module-2", "mock-plugin-3")
                                    .stop_plugin("mock-module-2", "mock-plugin-4")
                                    .start_plugin("mock-module-1", "mock-plugin-1")
                                    .start_plugin("mock-module-1", "mock-plugin-2")
                                    .start_plugin("mock-module-2", "mock-plugin-3")
                                    .start_plugin("mock-module-2", "mock-plugin-5")
                                    .start_plugin("mock-module-3", "mock-plugin-6");
    {
        status.set_plugin_status(&PluginId::new("mock-module-1", "mock-plugin-1"), PluginStatusEnum::Inactive);
        status.set_plugin_status(&PluginId::new("mock-module-1", "mock-plugin-2"), PluginStatusEnum::Active);
        status.set_plugin_status(&PluginId::new("mock-module-2", "mock-plugin-3"), PluginStatusEnum::Active);

        let stop_list:Vec<(&PluginId, &Arc<str>)> = PluginModulesUtils::validate_and_get_plugins_for_action(
                                                                    &mut modules, 
                                                                    &status, 
                                                                    &plugin_configuration, 
                                                                    PluginActionEnum::Start, 
                                                                    &mut erorr_log)
                                                                .into_iter()
                                                                .map(|(plugin_id, plugin)| (plugin_id, plugin.get_plugin_name()))
                                                                .collect();
                                                                
        assert_eq!(vec![(&PluginId::new("mock-module-1", "mock-plugin-1"), &new_str("mock-plugin-1")),
                        (&PluginId::new("mock-module-2", "mock-plugin-5"), &new_str("mock-plugin-5")),
                        (&PluginId::new("mock-module-3", "mock-plugin-6"), &new_str("mock-plugin-6"))
        ], stop_list);

        assert_eq!(vec![PluginManagerError::PluginAlreadyStarted(PluginId::new("mock-module-1", "mock-plugin-2")),
                        PluginManagerError::PluginAlreadyStarted(PluginId::new("mock-module-2", "mock-plugin-3"))
        ], *erorr_log.get_warnings());

        assert_eq!(true, erorr_log.is_completed());
    }
}

#[test]
fn it_get_plugins_stop_warnings() {
    let mut modules = create_modules_data();
    let mut status = PluginManagerStatusData::new();
    let mut erorr_log = ErrorLog::new();

    let plugin_configuration = PluginConfiguration::new()
                                    .stop_plugin("mock-module-1", "mock-plugin-1")
                                    .stop_plugin("mock-module-1", "mock-plugin-2")
                                    .stop_plugin("mock-module-2", "mock-plugin-3")
                                    .stop_plugin("mock-module-2", "mock-plugin-4")
                                    .start_plugin("mock-module-1", "mock-plugin-1")
                                    .start_plugin("mock-module-1", "mock-plugin-2")
                                    .start_plugin("mock-module-2", "mock-plugin-3")
                                    .start_plugin("mock-module-2", "mock-plugin-5")
                                    .start_plugin("mock-module-3", "mock-plugin-6");
    {
        status.set_plugin_status(&PluginId::new("mock-module-1", "mock-plugin-1"), PluginStatusEnum::Inactive);
        status.set_plugin_status(&PluginId::new("mock-module-1", "mock-plugin-2"), PluginStatusEnum::Active);
        status.set_plugin_status(&PluginId::new("mock-module-2", "mock-plugin-3"), PluginStatusEnum::Active);

        let stop_list:Vec<(&PluginId, &Arc<str>)> = PluginModulesUtils::validate_and_get_plugins_for_action(
                                                                    &mut modules, 
                                                                    &status, 
                                                                    &plugin_configuration, 
                                                                    PluginActionEnum::Stop, 
                                                                    &mut erorr_log)
                                                                .into_iter()
                                                                .map(|(plugin_id, plugin)| (plugin_id, plugin.get_plugin_name()))
                                                                .collect();
                                                                
        assert_eq!(vec![(&PluginId::new("mock-module-1", "mock-plugin-2"), &new_str("mock-plugin-2")),
                        (&PluginId::new("mock-module-2", "mock-plugin-3"), &new_str("mock-plugin-3"))
        ], stop_list);

        assert_eq!(vec![PluginManagerError::PluginNotStarted(PluginId::new("mock-module-1", "mock-plugin-1")),
                        PluginManagerError::PluginNotStarted(PluginId::new("mock-module-2", "mock-plugin-4"))
        ], *erorr_log.get_warnings());

        assert_eq!(true, erorr_log.is_completed());
    }
}

#[test]
fn it_test_no_duplicated_mut_ref() {
    let mut modules = create_modules_data();
    let mut status = PluginManagerStatusData::new();
    let mut erorr_log = ErrorLog::new();

    let plugin_configuration = PluginConfiguration::new()
                                    .stop_plugin("mock-module-1", "mock-plugin-1")
                                    .stop_plugin("mock-module-1", "mock-plugin-2")
                                    .stop_plugin("mock-module-1", "mock-plugin-1")
                                    .start_plugin("mock-module-2", "mock-plugin-3")
                                    .start_plugin("mock-module-2", "mock-plugin-5")
                                    .start_plugin("mock-module-2", "mock-plugin-3");

    {
        let stop_list:Vec<(&PluginId, &Arc<str>)> = PluginModulesUtils::validate_and_get_plugins_for_action(
                                                                    &mut modules, 
                                                                    &status, 
                                                                    &plugin_configuration, 
                                                                    PluginActionEnum::Start, 
                                                                    &mut erorr_log)
                                                                .into_iter()
                                                                .map(|(plugin_id, plugin)| (plugin_id, plugin.get_plugin_name()))
                                                                .collect();
                                                                
        assert_eq!(vec![(&PluginId::new("mock-module-2", "mock-plugin-3"), &new_str("mock-plugin-3")),
                        (&PluginId::new("mock-module-2", "mock-plugin-5"), &new_str("mock-plugin-5"))
        ], stop_list);

        assert_eq!(true, erorr_log.is_no_alerts());
    }

    {
        status.set_plugin_status(&PluginId::new("mock-module-1", "mock-plugin-1"), PluginStatusEnum::Active);
        status.set_plugin_status(&PluginId::new("mock-module-1", "mock-plugin-2"), PluginStatusEnum::Active);
        

        let stop_list:Vec<(&PluginId, &Arc<str>)> = PluginModulesUtils::validate_and_get_plugins_for_action(
                                                                    &mut modules, 
                                                                    &status, 
                                                                    &plugin_configuration, 
                                                                    PluginActionEnum::Stop, 
                                                                    &mut erorr_log)
                                                                .into_iter()
                                                                .map(|(plugin_id, plugin)| (plugin_id, plugin.get_plugin_name()))
                                                                .collect();
                                                                
        assert_eq!(vec![(&PluginId::new("mock-module-1", "mock-plugin-1"), &new_str("mock-plugin-1")),
                        (&PluginId::new("mock-module-1", "mock-plugin-2"), &new_str("mock-plugin-2"))
        ], stop_list);

        assert_eq!(true, erorr_log.is_no_alerts());
    }
}