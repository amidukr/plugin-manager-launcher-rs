extern crate plugin_launcher;

use plugin_launcher::plugin::helpers::langutils::new_str;

use plugin_launcher::plugin::api::error::PluginManagerError;
use plugin_launcher::plugin::api::error::ErrorLog;

#[test]
fn it_test_succesfull() {
    let mut empty_log = ErrorLog::new();

    assert_eq!(Vec::<PluginManagerError>::new(), *empty_log.get_warnings());
    assert_eq!(Vec::<PluginManagerError>::new(), *empty_log.get_errors());

    
    assert_eq!(true, empty_log.is_completed());
    assert_eq!(true, empty_log.is_no_alerts());
    assert_eq!(false, empty_log.is_failed());
}

#[test]
fn it_test_passed() {
    let mut empty_log = ErrorLog::new();

    empty_log.add_warning(PluginManagerError::CustomError(new_str("Just warning")));

    assert_eq!(vec![PluginManagerError::CustomError(new_str("Just warning"))], *empty_log.get_warnings());
    assert_eq!(Vec::<PluginManagerError>::new(), *empty_log.get_errors());

    
    assert_eq!(true, empty_log.is_completed());
    assert_eq!(false, empty_log.is_no_alerts());
    assert_eq!(false, empty_log.is_failed());
}

#[test]
fn it_test_error() {
    let mut empty_log = ErrorLog::new();

    empty_log.add_error(PluginManagerError::CustomError(new_str("Error")));

    assert_eq!(Vec::<PluginManagerError>::new(), *empty_log.get_warnings());
    assert_eq!(vec![PluginManagerError::CustomError(new_str("Error"))], *empty_log.get_errors());
    
    assert_eq!(false, empty_log.is_completed());
    assert_eq!(false, empty_log.is_no_alerts());
    assert_eq!(true, empty_log.is_failed());
}

#[test]
fn it_test_error_no_duplicates() {
    let mut log = ErrorLog::new();

    log.add_error(PluginManagerError::CustomError(new_str("Error1")));
    log.add_error(PluginManagerError::CustomError(new_str("Error2")));
    log.add_error(PluginManagerError::CustomError(new_str("Error1")));

    assert_eq!(Vec::<PluginManagerError>::new(), *log.get_warnings());
    assert_eq!(vec![PluginManagerError::CustomError(new_str("Error1")),
                    PluginManagerError::CustomError(new_str("Error2"))
    ], *log.get_errors());
    
    assert_eq!(false, log.is_completed());
    assert_eq!(false, log.is_no_alerts());
    assert_eq!(true, log.is_failed());
}

#[test]
fn it_test_warning_no_duplicates() {
    let mut log = ErrorLog::new();

    log.add_warning(PluginManagerError::CustomError(new_str("Error1")));
    log.add_warning(PluginManagerError::CustomError(new_str("Error2")));
    log.add_warning(PluginManagerError::CustomError(new_str("Error1")));

    assert_eq!(Vec::<PluginManagerError>::new(), *log.get_errors());
    assert_eq!(vec![PluginManagerError::CustomError(new_str("Error1")),
                    PluginManagerError::CustomError(new_str("Error2"))
    ], *log.get_warnings());
    
    assert_eq!(true, log.is_completed());
    assert_eq!(false, log.is_no_alerts());
    assert_eq!(false, log.is_failed());
}