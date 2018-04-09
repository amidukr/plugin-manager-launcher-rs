use std::collections::HashMap;
use std::cmp::Eq;
use std::hash::Hash;

use std::sync::RwLock;

use std::sync::Arc;

use plugin::api::context::*;

fn map_lazy_get<'a, K: Eq + Hash + Clone, V, F: FnOnce() -> V>(hash_map: &'a mut HashMap<K, V>, key: &K, value_supplier: F) -> &'a mut V {
        if hash_map.contains_key(key) {
            return hash_map.get_mut(key).unwrap();
        }

        hash_map.insert(key.clone(), value_supplier());

        return hash_map.get_mut(key).unwrap();
}
pub struct SharedPluginManager {
    components: RwLock<HashMap< Arc<String>, Vec<PlguinComponent> >>,
    components_arc_cache: RwLock<HashMap< Arc<String>, Arc<Vec<PlguinComponent>> >>
}



impl SharedPluginManager {
    pub fn new() -> SharedPluginManager {
        return SharedPluginManager {
            components: RwLock::new(HashMap::new()),
            components_arc_cache: RwLock::new(HashMap::new())
        };
    }

    fn add_component(&self, lookup_key: &Arc<String>, component: PlguinComponent) {
        let plugin_map = &mut *self.components.write().unwrap();
        let cache = &mut *self.components_arc_cache.write().unwrap();

        let components = map_lazy_get(plugin_map, lookup_key, || Vec::new());

        cache.remove(lookup_key);
        components.push(component);
    }

    fn get_components(&self, lookup_key: &Arc<String>) -> Arc<Vec<PlguinComponent>> {
        {
            let cache = &*self.components_arc_cache.read().unwrap();
            if let Some(result) = cache.get(lookup_key) {
                return result.clone();
            }
        }

        let cache = &mut *self.components_arc_cache.write().unwrap();

        map_lazy_get(cache, lookup_key, || {
            let plugin_map = self.components.read().unwrap();

            Arc::new(if let Some(result) = plugin_map.get(lookup_key) {
                result.clone()
            }else{
                Vec::new()
            })

        }).clone()
    }
}

pub struct LocalPluginManager{
    shared_manager: Arc<SharedPluginManager>
    //cache: HashMap<Arc<String>, Arc<Vec<Arc<Any>>> >
}

impl LocalPluginManager {
    pub fn new() -> LocalPluginManager {
        return LocalPluginManager {
            shared_manager: Arc::new(SharedPluginManager::new())
        };
    }
}

impl PluginManager for LocalPluginManager {
    fn add_component(&self, lookup_key: &Arc<String>, component: PlguinComponent) {
        self.shared_manager.add_component(lookup_key, component)
    }

    fn get_components(&self, lookup_key: &Arc<String>) -> Arc<Vec<PlguinComponent>> {
        self.shared_manager.get_components(lookup_key)
    }
}
