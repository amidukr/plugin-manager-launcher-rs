use std::collections::{HashMap, HashSet};

use std::hash::Hash;
use std::fmt::Display;
use std::any::Any;

use std::mem::transmute;

pub fn get_hashmap_values<'a,'b, K: 'b + Eq + Hash, V, II>(map: &'a mut HashMap<K, V>, keys: II) -> Result<Vec<&'a mut V>, String>
where II: IntoIterator<Item=&'b K>
{
    let mut result = Vec::new();
    let mut duplicated_key_set = HashSet::new();

    let ptr_map = map as *mut HashMap<K,V>;

    for key in keys {
        unsafe {
            
            if duplicated_key_set.contains(key) {
                return Result::Err("Duplicated keys found".to_string());
            }

            duplicated_key_set.insert(key);
        
        
            let second_hash_map: &'a mut HashMap<K, V> = transmute(ptr_map);
            let value_option = second_hash_map.get_mut(key);
        

            if value_option.is_none() {
                return Result::Err("Key not found".to_string());
            }

            result.push(value_option.unwrap());
        }
    }

    return Result::Ok(result);
}
