use anyhow::Result;

use crate::core::{component::Component, component_ref::ComponentRef};

pub trait GameObject {
    fn get_component<T: Component>() -> Option<ComponentRef<'static, T>>;
    fn get_components<T: Component>() -> Vec<ComponentRef<'static, T>>;

    fn add_component<T: Component>(&self, component: T);
    fn add_component_ref<T: Component>(&self, component: ComponentRef<T>);

    fn remove_component<T: Component>(&self, component: T) -> Result<T>;
    fn remove_component_ref<T: Component>(&self, component: ComponentRef<T>) -> Result<T>;
}