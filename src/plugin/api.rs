use std::any::Any;
use std::marker::PhantomData;

use std::sync::RwLock;
use std::collections::HashMap;

use std::sync::Arc;
use std::sync::atomic::AtomicPtr;

use std::cmp::Eq;
use std::hash::Hash;

use std::ops::Deref;

//TODO: move to collection lib
fn map_lazy_get<'a, K: Eq + Hash + Clone, V, F: FnOnce() -> V>(hash_map: &'a mut HashMap<K, V>, key: &K, value_supplier: F) -> &'a mut V {
        if hash_map.contains_key(key) {
            return hash_map.get_mut(key).unwrap();
        }

        hash_map.insert(key.clone(), value_supplier());

        return hash_map.get_mut(key).unwrap();
}

pub trait LookupKeyInterface<T: Any + ?Sized> {
    fn key_name(&self) -> &Arc<String>;
}

pub struct LookupKey<T: Any + ?Sized> {
    phantom_data: PhantomData<AtomicPtr<Box<T>>>,
    key_name: Arc<String>
}

impl <T: Any + ?Sized + > LookupKeyInterface<T> for LookupKey<T> {
    fn key_name(&self) -> &Arc<String> {
        return &self.key_name;
    }
}

impl <T: ?Sized + Any> LookupKey<T> {
    pub fn from_string(key_name: String) -> LookupKey<T> {
        return LookupKey {
            phantom_data: PhantomData,
            key_name: Arc::new(key_name)
        };
    }

    pub fn from_str(key_name: &str) -> LookupKey<T> {
         return LookupKey::from_string(key_name.to_owned());
    }

    pub fn take(&self) -> &Self {
        return self;
    }
}


pub trait PluginManager {
    fn add_component(&self, lookup_key: &Arc<String>, component: PlguinComponent);
    fn get_components(&self, lookup_key: &Arc<String>) -> Arc<Vec<PlguinComponent>>;
}

impl PluginManager {
    pub fn register_unsized<T: Any + Sync + Send + ?Sized>(&self, lookup_key: &LookupKeyInterface<T>, component: Arc<T>) {
        self.add_component(lookup_key.key_name(), Arc::new(component));
    }

    pub fn register_trait<T: Any + Sync + Send + ?Sized>(&self, lookup_key: &LookupKeyInterface<T>, component: Arc<T>) {
        self.register_unsized(lookup_key, component)
    }

    pub fn register_sized<T: Any + Sync + Send + Sized>(&self, lookup_key: &LookupKeyInterface<T>, component: T) {
        //self.add_component(lookup_key.key_name(), Arc::new(component) as Arc<T>);
        self.register_unsized(lookup_key, Arc::new(component));
    }


    pub fn lookup_components<T: Any + Sync + Send + ?Sized>(&self, lookup_key: &LookupKeyInterface<T>) -> Vec<Arc<T>> {
        let components = self.get_components(lookup_key.key_name());
        let mut result: Vec<Arc<T>> = Vec::new();

        for component in components.deref() {
            let any_ref:&Any = component.deref();
            if let Some(value) = any_ref.downcast_ref::<Arc<T>>() {
                
                result.push(value.clone());
            }
        }

        return result;
    }
}

type PlguinComponent = Arc<Any + Send + Sync>;

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
