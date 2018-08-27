use std::collections::{HashMap, HashSet};

use std::hash::Hash;
use std::fmt::Display;
use std::any::Any;

pub fn get_hashmap_values<'a, K: Eq + Hash, V>(map: &'a mut HashMap<K, V>, keys: &Vec<K>) -> Result<Vec<&'a V>, String>
{
    let mut result = Vec::new();
    let mut duplicated_key_set = HashSet::new();

    for key in keys {
        if duplicated_key_set.contains(key) {
            return Result::Err("Duplicated keys found".to_string());
        }

        duplicated_key_set.insert(key);

        let value_option = map.get(key);

        if value_option.is_none() {
            return Result::Err("Key not found".to_string());
        }

        result.push(value_option.unwrap());
    }

    return Result::Ok(result);
}