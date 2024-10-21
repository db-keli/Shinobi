use std::any::{Any, TypeId};
use std::collections::HashMap;

pub struct ServiceLocator {
    services: HashMap<TypeId, Box<dyn Any>>,
}

impl ServiceLocator {
    pub fn new() -> Self {
        ServiceLocator {
            services: HashMap::new(),
        }
    }

    pub fn register<T: Any>(&mut self, service: T) {
        self.services.insert(TypeId::of::<T>(), Box::new(service));
    }

    pub fn get<T: Any>(&self) -> Option<&T> {
        self.services
            .get(&TypeId::of::<T>())
            .and_then(|boxed| boxed.downcast_ref::<T>())
    }
}
