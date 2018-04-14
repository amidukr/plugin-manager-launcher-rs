use std::any::Any;
use std::marker::PhantomData;

use std::ops::Deref;
use std::sync::Arc;
use std::sync::atomic::AtomicPtr;

pub trait LookupKeyInterface<T: Any + ?Sized> {
    fn key_name(&self) -> &Arc<String>;
}

pub struct LookupKey<T: Any + ?Sized> {
    phantom_data: PhantomData<AtomicPtr<Box<T>>>,
    key_name: Arc<String>
}

impl <T: Any + ?Sized> LookupKeyInterface<T> for LookupKey<T> {
    fn key_name(&self) -> &Arc<String> {
        return &self.key_name;
    }
}

impl <T: Any + ?Sized> LookupKey<T> {
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


pub type PlguinComponent = Arc<Any + Send + Sync>;
pub trait PluginContainer {
    fn add_component(&self, lookup_key: &Arc<String>, component: PlguinComponent);
    fn get_components(&self, lookup_key: &Arc<String>) -> Arc<Vec<PlguinComponent>>;
}

impl PluginContainer {
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
