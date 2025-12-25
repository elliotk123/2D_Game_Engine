use engine_common::{
    engine_bus::{EngineBus, PhysicsToLogicChannel, SysInToGameLogicChannel},
    engine_module::EngineModule
};

use super::game_logic_io::{GameLogicInputs, GameLogicOutputs};
use super::game_logic_conf::{GameLogicState, process};


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
        for msg in bus.sysin_to_logic.rx.try_iter(){
            match msg{
                SysInToGameLogicChannel::KeyboardEvents 
                { 
                    events 
                }=>
                {
                    self.inputs.keyboard_events = events
                }
                _ =>
                {
                    break;
                }
            }
        }
        for msg in bus.physics_to_logic.rx.try_iter(){
            match msg{
                PhysicsToLogicChannel::Collision 
                { 
                    index_a, index_b, 
                    pocx, pocy, angle, 
                    depth 
                }=>
                {
                    self.inputs.collision_events += 1;
                }
            }
        }       
    }

    fn write_output_messages(&mut self, bus: &mut EngineBus){

    }
}

impl EngineModule for GameLogicModule{
    fn run(&mut self, bus : &mut EngineBus){
        // read input channels
        self.read_input_messages(bus);
        process(self.inputs.clone(), &mut self.state_data, &mut self.outputs);
        self.write_output_messages(bus);
    }
}