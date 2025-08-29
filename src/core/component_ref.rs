use std::marker::PhantomData;

use crate::core::{component::{Component, ComponentIdentifier}, component_arena::ComponentArena};

#[derive(Debug, Clone, Copy)]
pub struct ComponentRef<'arena, T> {
    p: PhantomData<T>,
    arena: &'arena ComponentArena,
    id: ComponentIdentifier,
}

impl<'arena, T: Component + 'static> ComponentRef<'arena, T> {
    pub fn new(arena: &'arena ComponentArena, id: ComponentIdentifier) -> Self {
        ComponentRef { p: PhantomData, arena, id}
    }
    pub fn get(&self) -> Option<&T> {
        self.arena.get_component(&self.id)
    }

    pub fn get_mut(&self) -> Option<&mut T> {
        self.arena.get_component_mut(&self.id)
    }
}