use engine_core::engine_module::EngineModule;

struct CompositorModule{
    id : u64
}

impl CompositorModule{
    fn new() -> CompositorModule{
        CompositorModule{
            id : 0
        }
    }
}

impl EngineModule for CompositorModule{
    fn run(&mut self){
        self.id = self.id + 1;
    }
}

