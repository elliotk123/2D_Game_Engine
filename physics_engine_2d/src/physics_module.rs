use super::entity::Entity;
use engine_common::{engine_bus::EngineBus, engine_module::EngineModule};

pub struct PhysicsModule {
    entities : Vec<Entity>,
    delta_t_s : f32,
}

impl PhysicsModule
{
    pub fn new(delta_t : f32, )->PhysicsModule
    {
        PhysicsModule
        {
            entities : Vec::new(),
            delta_t_s : delta_t
        }

    }
}

impl EngineModule for PhysicsModule
{
    fn run(&mut self, bus : &mut EngineBus)
    {
        for entity in self.entities.iter_mut()
        {
            entity.update(self.delta_t_s);
        }
    }
}