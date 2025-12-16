use super::engine_module::{ModuleId, EngineModule};
use super::engine_bus::{EngineBus};
use std::collections::HashMap;
use physics_engine_2d::physics_module::PhysicsModule;
use graphics_compositor_2d::compositor_module::CompositorModule;

pub struct GameState {
    // modules: HashMap<ModuleId, Box<dyn EngineModule>>
    physics_2d : PhysicsModule,
    compositor_2d : CompositorModule,

}

impl GameState{
    pub fn new(engine_bus : EngineBus) -> GameState
    {
        GameState { 
            physics_2d: PhysicsModule::new(), 
            compositor_2d: CompositorModule::new()
        }
    }


    pub fn run(&mut self, module_id : ModuleId)
    {
        match module_id
        {
            ModuleId::PHYS2D => {self.physics_2d.run()}
            ModuleId::COMP2D => {self.compositor_2d.run()}
        }
    }
}

