extern crate plugin_launcher;

use std::collections::HashMap;
use plugin_launcher::plugin::helpers::collections::get_hashmap_values;

fn create_test_hash_map() -> HashMap<String, String> 
{
    let mut map = HashMap::new();

    map.insert("1".to_string(), "one".to_string());
    map.insert("2".to_string(), "two".to_string());
    map.insert("3".to_string(), "three".to_string());
    map.insert("4".to_string(), "four".to_string());
    map.insert("5".to_string(), "five".to_string());

    return map;
}

#[test]
fn it_get_hashmap_values() {
    let mut map = create_test_hash_map();

    let values = get_hashmap_values(&mut map, &vec!["2".to_string(), 
                                                    "3".to_string(),
                                                    "5".to_string()]);

    
    assert_eq!( vec!["two", "three", "five"], values.unwrap());
    
}

#[test]
fn it_get_hashmap_values_key_not_found() {
    let mut map = create_test_hash_map();

    let values = get_hashmap_values(&mut map, &vec!["2".to_string(), 
                                                    "3-three".to_string(),
                                                    "5".to_string()]);

    assert_eq!( Result::Err("Key not found".to_string()), values);
}

#[test]
fn it_get_hashmap_values_duplicated_keys() {
    let mut map = create_test_hash_map();

    let values = get_hashmap_values(&mut map, &vec!["2".to_string(), 
                                                    "2".to_string(),
                                                    "5".to_string()]);

    assert_eq!( Result::Err("Duplicated keys found".to_string()), values);
}
