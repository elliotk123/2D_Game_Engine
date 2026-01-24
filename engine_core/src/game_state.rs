use std::time::Duration;

use engine_common::{
    engine_module::{ModuleId, EngineModule, EngineIoModule},
    engine_bus::EngineBus,
    game_conf::GameConf
};
use game_logic::game_logic_module::GameLogicModule;
use physics_engine_2d::physics_module::PhysicsModule;
use graphics_compositor_2d::compositor_module::CompositorModule;
use render_sync::render_sync_module::RenderSyncModule;

use super::engine_io::EngineIO;

pub struct GameState<T : GameConf> {
    // modules: HashMap<ModuleId, Box<dyn EngineModule>>
    engine_bus    : EngineBus,
    physics_2d    : PhysicsModule,
    render_sync   : RenderSyncModule,
    compositor_2d : CompositorModule,
    game_logic    : GameLogicModule<T>,
    engine_io     : EngineIO,
}

impl<T : GameConf> GameState<T>{
    pub fn new(delta_t : Duration) -> GameState<T>
    {
        GameState { 
            engine_bus    : EngineBus::new(),
            physics_2d    : PhysicsModule::new(delta_t), 
            compositor_2d : CompositorModule::new(1000,500),
            render_sync   : RenderSyncModule::new(),
            engine_io     : EngineIO::new(),
            game_logic    : GameLogicModule::new()
        }
    }


    pub fn run(&mut self, module_id : ModuleId)->bool
    {
        match module_id
        {
            ModuleId::PHYS2D => {self.physics_2d.run(&mut self.engine_bus)}
            ModuleId::COMP2D => {self.compositor_2d.run(&mut self.engine_bus)}
            ModuleId::RENDER => {self.render_sync.run(&mut self.engine_bus)}
            ModuleId::SYSIN  => {self.engine_io.read_input(&mut self.engine_bus)}
            ModuleId::SYSOUT => {self.engine_io.write_output(&mut self.engine_bus)}
            ModuleId::LOGIC  => {self.game_logic.run(&mut self.engine_bus)}
        }
    }
}

