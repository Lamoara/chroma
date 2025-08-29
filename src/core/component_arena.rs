use std::{any::{Any, TypeId}, cell::RefCell, collections::HashMap};

use uuid::Uuid;

use crate::core::{component::{Component, ComponentIdentifier}, component_ref::ComponentRef};

type ComponentMap = RefCell<HashMap<ComponentIdentifier, (TypeId, Box<dyn Any>)>>;

#[derive(Debug)]
pub struct ComponentArena {
    components: ComponentMap,
}

impl ComponentArena {
    pub fn add_component<T: Component + 'static>(&self, component: T) -> ComponentRef<T> {
        let id = Uuid::new_v4();
        self.components
            .borrow_mut()
            .insert(id, (TypeId::of::<T>(), Box::new(component)));
        ComponentRef::new(self, id)
    }

    //TODO(lamoara) add feedback to know if the component has been removed
    pub fn remove_component(&self, id: &ComponentIdentifier) {
        self.components.borrow_mut().remove(id);
    }

    pub fn get_component<T: Component + 'static>(&self, id: &ComponentIdentifier) -> Option<&T> {
        let binding = self.components.borrow();
        binding.get(id).and_then(|(t, b)| {
            if TypeId::of::<T>() == *t {
                Some(unsafe { &*((&**b) as *const _ as *const T) })
            } else {
                None
            }
        })
    }

    #[allow(clippy::mut_from_ref)]
    pub fn get_component_mut<T: Component + 'static>(&self, id: &ComponentIdentifier) -> Option<&mut T> {
        let mut binding = self.components.borrow_mut();
        binding.get_mut(id).and_then(|(t, b)| {
            if TypeId::of::<T>() == *t {
                Some(unsafe { &mut *((&mut **b) as *mut _ as *mut T) })
            } else {
                None
            }
        })
    }
}