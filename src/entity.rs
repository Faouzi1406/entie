use crate::world::World;

pub struct Entity<'w> {
    pub(crate) id: usize,
    pub(crate) world: &'w mut World,
}

impl<'w> Entity<'w> {
    pub(crate) fn new(id: usize, world: &'w mut World) -> Self {
        Self { id, world }
    }

    pub fn add_component<ComponentType: 'static>(&mut self, component: ComponentType) {
        self.world.add_component(self.id, component);
    }
}
