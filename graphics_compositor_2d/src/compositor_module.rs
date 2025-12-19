use engine_common::{
    engine_module::EngineModule,
    engine_bus::EngineBus
};

pub struct CompositorModule{
    id : u64
}

impl CompositorModule{
    pub fn new() -> CompositorModule{
        CompositorModule{
            id : 0
        }
    }
}

impl EngineModule for CompositorModule{
    fn run(&mut self, bus : &mut EngineBus){
        self.id = self.id + 1;
    }
}

