use super::entity::Entity;
use engine_core::engine_module::EngineModule;

struct PhysicsModule {
    entities : Vec<Entity>,
    delta_t_s : f32,
    
}

impl PhysicsModule
{
    fn new(delta_t : f32, )->PhysicsModule
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
    fn run(&self)
    {
        for entity in self.entities.iter()
        {
            entity.update(self.delta_t_s);
        }
    }
}