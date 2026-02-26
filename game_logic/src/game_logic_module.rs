use engine_common::{
    engine_bus::{EngineBus, SysInToGameLogicChannel},
    engine_module::EngineModule,
    game_conf::{GameConf, GameLogicInputs, GameLogicOutputs}
};

#[derive(Clone)]
pub struct GameLogicModule<T : GameConf>{
    inputs : GameLogicInputs,
    game_conf : T,
    outputs : GameLogicOutputs
}

impl<T : GameConf> GameLogicModule<T>{
    pub fn new() -> GameLogicModule<T>{
        GameLogicModule{
            inputs : GameLogicInputs::new(),
            game_conf : T::new(),
            outputs : GameLogicOutputs::new()
        }
    }

    fn read_input_messages(&mut self, bus :  &mut EngineBus)
    {
        // remove states and inputs from previous frame
        self.inputs.physics_events.clear();
        self.inputs.physics_states.clear();

        // scan for new messages
        while let Ok(msg) = bus.sysin_to_logic.rx.try_recv(){
            match msg{
                SysInToGameLogicChannel::KeyboardEvents 
                { 
                    events 
                }=>
                {
                    self.inputs.keyboard_events = events;
                    //println!("KEYBOARD EVENTS");
                }
                _ =>
                {
                    break;
                }
            }
        }
        while let Ok(msg) = bus.physics_state_to_logic.rx.try_recv() {
            self.inputs.physics_states.push(msg);
        }
        while let Ok(msg) = bus.physics_event_to_logic.rx.try_recv(){
            self.inputs.physics_events.push(msg);
        }     
    }

    fn write_output_messages(&mut self, bus: &mut EngineBus){
        for msg in self.outputs.physics_commands.iter(){
            bus.logic_to_physics.tx.send(msg.clone()).unwrap();
        }
        for msg in self.outputs.render_sync_commands.iter(){
            bus.logic_to_render_sync.tx.send(msg.clone()).unwrap();
        }
        // Clear the internal buffers
        self.outputs.physics_commands.clear();
        self.outputs.render_sync_commands.clear();
    }
}

impl<T : GameConf> EngineModule for GameLogicModule<T>{
    fn run(&mut self, bus : &mut EngineBus)->bool{
        // read input channels
        self.read_input_messages(bus);
        let result = self.game_conf.process(&self.inputs, &mut self.outputs);
        if result == false {
            return false;
        }
        self.write_output_messages(bus);
        return true;
    }
}