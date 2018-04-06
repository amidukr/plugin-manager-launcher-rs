extern crate plugin_launcher;

#[macro_use]
extern crate lazy_static;

use std::sync::Arc;


use plugin_launcher::plugin::api::*;



struct A{}
struct B{}

trait TA  {}
trait TA1 {}
trait TB  {}
trait TC  {}

impl TA  for A {}
impl TA1 for A {}
impl TB  for B {}

impl TC for A {}
impl TC for B {}

lazy_static! {
    static ref TA_KEY: LookupKeyValue<TA> = { return LookupKeyValue::from_str("ta-key") };
    static ref TB_KEY: LookupKeyValue<TB> = { return LookupKeyValue::from_str("tb-key") };
    static ref TC_KEY: LookupKeyValue<TC> = { return LookupKeyValue::from_str("tc-key") };
}

#[test]
fn it_test_core_save_function() {
    let plugin_manager: &PluginManager = &LocalPluginManager::new();

    plugin_manager.add_component(&Arc::new("key1".to_owned()), Arc::new("value1".to_owned()));
    plugin_manager.add_component(&Arc::new("key2".to_owned()), Arc::new("value2".to_owned()));
    plugin_manager.add_component(&Arc::new("key2".to_owned()), Arc::new("value3".to_owned()));

    let key1_list = plugin_manager.get_components(&Arc::new("key1".to_owned()));
    let key2_list = plugin_manager.get_components(&Arc::new("key2".to_owned()));
    let key3_list = plugin_manager.get_components(&Arc::new("key3".to_owned()));

    assert_eq!(1, key1_list.len());
    assert_eq!(2, key2_list.len());
    assert_eq!(0, key3_list.len());
}

#[test]
fn it_trait_test() {
    let plugin_manager: &PluginManager = &LocalPluginManager::new();
    
    plugin_manager.register_trait(TA_KEY.take(), Arc::new(A{}));
    plugin_manager.register_trait(TB_KEY.take(), Arc::new(B{}));

    plugin_manager.register_trait(TC_KEY.take(), Arc::new(A{}));
    plugin_manager.register_trait(TC_KEY.take(), Arc::new(B{}));
    
    let ta_list: Vec<Arc<TA>> = plugin_manager.lookup_components(TA_KEY.take());
    let tb_list: Vec<Arc<TB>> = plugin_manager.lookup_components(TB_KEY.take());
    let tc_list: Vec<Arc<TC>> = plugin_manager.lookup_components(TC_KEY.take());

    assert_eq!(1, ta_list.len());
    assert_eq!(1, tb_list.len());
    assert_eq!(2, tc_list.len());
}

#[test]
fn it_eq_test() {
    let plugin_manager: &PluginManager = &LocalPluginManager::new();

    let str_key_1: &LookupKeyValue<String> = &LookupKeyValue::from_str("str-key1");
    let str_key_2: &LookupKeyValue<String> = &LookupKeyValue::from_str("str-key2");
    let str_key_3: &LookupKeyValue<String> = &LookupKeyValue::from_str("str-key3");

    plugin_manager.register_sized(str_key_1, "key1-value1".to_owned());
    
    plugin_manager.register_sized(str_key_2, "key2-value1".to_owned());
    plugin_manager.register_sized(str_key_2, "key2-value2".to_owned());
    
    
    let key1_list: Vec<Arc<String>> = plugin_manager.lookup_components(str_key_1);
    let key2_list: Vec<Arc<String>> = plugin_manager.lookup_components(str_key_2);
    let key3_list: Vec<Arc<String>> = plugin_manager.lookup_components(str_key_3);

    assert_eq!(1, key1_list.len());
    assert_eq!(2, key2_list.len());
    assert_eq!(0, key3_list.len());



    assert_eq!(vec!(Arc::new("key1-value1".to_owned())), key1_list);
    assert_eq!(vec!(Arc::new("key2-value1".to_owned()), Arc::new("key2-value2".to_owned())), key2_list);
    assert_eq!(Vec::<Arc<String>>::new(), key3_list);
}

#[test]
fn it_same_struct_test() {
    let plugin_manager: &PluginManager = &LocalPluginManager::new();

    let ta_key:  &LookupKeyValue<TA>  = &LookupKeyValue::from_str("same-struct");
    let ta1_key: &LookupKeyValue<TA1> = &LookupKeyValue::from_str("same-struct");

    plugin_manager.register_trait(ta_key, Arc::new(A{}));
    plugin_manager.register_trait(ta1_key, Arc::new(A{}));

    let ta_list:  Vec<Arc<TA>>  = plugin_manager.lookup_components(ta_key);
    let ta1_list: Vec<Arc<TA1>> = plugin_manager.lookup_components(ta1_key);
    let all_list = plugin_manager.get_components(&Arc::new("same-struct".to_owned()));

    assert_eq!(1, ta_list.len());
    assert_eq!(1, ta1_list.len());
    assert_eq!(2, all_list.len());
}


#[test]
fn it_key_different_type_test() {
    let plugin_manager: &PluginManager = &LocalPluginManager::new();

    let inject_str_key: &LookupKeyValue<String> = &LookupKeyValue::from_str("inject-test-key");
    let inject_ta_key: &LookupKeyValue<TA> = &LookupKeyValue::from_str("inject-test-key");

    plugin_manager.register_sized(inject_str_key, "inject-test-value".to_owned());
    plugin_manager.register_trait(inject_ta_key, Arc::new(A{}));
    plugin_manager.add_component(&Arc::new("inject-test-key".to_owned()), Arc::new(A{}));
    
    let str_list = plugin_manager.lookup_components(inject_str_key);
    let ta_list = plugin_manager.lookup_components(inject_ta_key);
    let all_list = plugin_manager.get_components(&Arc::new("inject-test-key".to_owned()));

    assert_eq!(1, str_list.len());
    assert_eq!(1, ta_list.len());
    assert_eq!(3, all_list.len());

    assert_eq!(vec!(Arc::new("inject-test-value".to_owned())), str_list);
}

#[test]
fn it_manual_inject_test() {
    let plugin_manager: &PluginManager = &LocalPluginManager::new();

    let inject_str_key: &LookupKeyValue<String> = &LookupKeyValue::from_str("manual-inject-key");
    let inject_ta_key:  &LookupKeyValue<TA> = &LookupKeyValue::from_str("manual-inject-key");

    plugin_manager.add_component(&Arc::new("manual-inject-key".to_owned()), Arc::new(Arc::new("value-string".to_owned())));
    plugin_manager.add_component(&Arc::new("manual-inject-key".to_owned()), Arc::new(Arc::new(A{}) as Arc<TA>));
    
    let str_list: Vec<Arc<String>> = plugin_manager.lookup_components(inject_str_key);
    let ta_list:  Vec<Arc<TA>>     = plugin_manager.lookup_components(inject_ta_key);
    let all_list: Arc<Vec<_>> = plugin_manager.get_components(&Arc::new("manual-inject-key".to_owned()));

    assert_eq!(1, str_list.len());
    assert_eq!(1, ta_list.len());
    assert_eq!(2, all_list.len());

    assert_eq!(vec!(Arc::new("value-string".to_owned())), str_list);
}
