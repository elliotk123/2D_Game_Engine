use engine_common::{
    engine_module::{ModuleId, EngineModule, EngineIoModule},
    engine_bus::{EngineBus}
};
use game_logic::game_logic_module::GameLogicModule;
use physics_engine_2d::physics_module::PhysicsModule;
use graphics_compositor_2d::compositor_module::CompositorModule;
use super::engine_io::EngineIO;

pub struct GameState {
    // modules: HashMap<ModuleId, Box<dyn EngineModule>>
    engine_bus : EngineBus,
    physics_2d : PhysicsModule,
    compositor_2d : CompositorModule,
    game_logic : GameLogicModule,
    engine_io : EngineIO,
}

impl GameState{
    pub fn new(engine_bus : EngineBus, delta_t_s : f32) -> GameState
    {
        GameState { 
            engine_bus,
            physics_2d: PhysicsModule::new(delta_t_s), 
            compositor_2d: CompositorModule::new(),
            engine_io : EngineIO::new(),
            game_logic : GameLogicModule::new()
        }
    }


    pub fn run(&mut self, module_id : ModuleId)
    {
        match module_id
        {
            ModuleId::PHYS2D => {self.physics_2d.run(&mut self.engine_bus)}
            ModuleId::COMP2D => {self.compositor_2d.run(&mut self.engine_bus)}
            ModuleId::SYSIN =>  {self.engine_io.read_input(&mut self.engine_bus)}
            ModuleId::SYSOUT => {self.engine_io.write_output(&mut self.engine_bus)}
            ModuleId::LOGIC =>  {self.game_logic.run(&mut self.engine_bus)}
        }
    }
}

