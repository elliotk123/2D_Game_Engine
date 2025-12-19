use engine_common::{
    engine_bus::EngineBus,
    engine_module::EngineModule
};


pub struct GameLogicModule{
    dummy : usize,
}

impl GameLogicModule{
    pub fn new() -> GameLogicModule{
        GameLogicModule {
             dummy: (0) 
        }
    }
}

impl EngineModule for GameLogicModule{
    fn run(&mut self, bus : &mut EngineBus){
        self.dummy = self.dummy + 1;
        if self.dummy >= 100 {
            self.dummy = 0
        }
    }
}