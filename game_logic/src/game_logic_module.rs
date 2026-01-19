use engine_common::{
    engine_bus::{EngineBus, SysInToGameLogicChannel},
    engine_module::EngineModule
};

use super::game_logic_io::{GameLogicInputs, GameLogicOutputs};
use super::game_logic_conf::{GameLogicState, process};

#[derive(Debug, Clone)]
pub struct GameLogicModule{
    inputs :GameLogicInputs,
    state_data : GameLogicState,
    outputs : GameLogicOutputs
}

impl GameLogicModule{
    pub fn new() -> GameLogicModule{
        GameLogicModule{
            inputs : GameLogicInputs::new(),
            state_data : GameLogicState::new(),
            outputs : GameLogicOutputs::new()
        }
    }

    fn read_input_messages(&mut self, bus :  &mut EngineBus)
    {
        // remove state inputs from previous frame
        self.inputs.physics_events.clear();
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
        while let Ok(msg) = bus.physics_to_logic.rx.try_recv(){
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

impl EngineModule for GameLogicModule{
    fn run(&mut self, bus : &mut EngineBus)->bool{
        // read input channels
        self.read_input_messages(bus);
        let result = process(&self.inputs, &mut self.state_data, &mut self.outputs);
        if result == false {
            return false;
        }
        self.write_output_messages(bus);
        return true;
    }
}