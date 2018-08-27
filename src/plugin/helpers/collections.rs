use std::collections::HashMap;
use std::hash::Hash;
use std::fmt::Display;
use std::any::Any;

fn format_key_not_found<K>(k: &K) -> String{
    format!("Key not found")
}

pub fn get_hashmap_values<'a, K: Eq + Hash, V>(map: &'a mut HashMap<K, V>, keys: &Vec<K>) -> Result<Vec<&'a V>, String>
{
    let mut result = Vec::new();

    for key in keys {
        let value_option = map.get(key);

        if value_option.is_none() {
            return Result::Err(format_key_not_found(key));
        }

        result.push(value_option.unwrap());
    }

    return Result::Ok(result);
}