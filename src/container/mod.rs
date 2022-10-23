pub mod factory;
pub mod settings;

use crate::container::factory::CreatedFromContainer;
use crate::container::settings::Settings;

use ahash::AHashMap;
use std::any::Any;
use std::any::TypeId;

#[derive(Debug)]
pub struct Container<'a> {
    pub settings: &'a Settings,
    map: AHashMap<TypeId, Box<dyn Any>>,
}

impl Container<'_> {
    pub fn new(settings: &Settings) -> Container {
        Container {
            settings,
            map: AHashMap::new(),
        }
    }

    pub fn get<T: CreatedFromContainer + 'static>(&mut self) -> T {
        let item = self
            .map
            .get(&TypeId::of::<T>())
            .and_then(|boxed| boxed.downcast_ref::<T>());

        if let Some(service) = item {
            return service.clone();
        }

        let service = T::create(&mut *self);

        self.map
            .insert(TypeId::of::<T>(), Box::new(service.clone()))
            .and_then(|boxed: Box<dyn Any>| -> Option<T> {
                boxed.downcast().ok().map(|boxed| *boxed)
            });

        service
    }
}
