extern crate plugin_launcher;

use std::collections::HashMap;
use std::sync::Arc;

use plugin_launcher::plugin::helpers::langutils::{get_hashmap_values, vec_of_str, new_str, sort_and_return};

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

#[test]
fn it_new_str() {
    let str_value: Arc<str> = new_str("string_value");

    assert_eq!(Arc::from("string_value"), str_value);
}

#[test]
fn it_vec_of_str() {
    let new_vector: Vec<Arc<str>> = vec_of_str(&["ab", "cd"]);

    assert_eq!(vec![new_str("ab"), new_str("cd")], new_vector);
}