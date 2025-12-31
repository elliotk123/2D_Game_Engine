use engine_common::{
    engine_bus::{EngineBus, PhysicsToLogicChannel, SysInToGameLogicChannel},
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
            self.inputs.physics_events.push(msg);
        }     
    }

    fn write_output_messages(&self, bus: &mut EngineBus){
        for msg in self.outputs.physics_commands.iter(){
            bus.logic_to_physics.tx.send(msg.clone());
        }
        for msg in self.outputs.render_sync_commands.iter(){
            bus.logic_to_render_sync.tx.send(msg.clone());
        }
    }
}

impl EngineModule for GameLogicModule{
    fn run(&mut self, bus : &mut EngineBus)->bool{
        // read input channels
        self.read_input_messages(bus);
        let result = process(self.inputs.clone(), &mut self.state_data, &mut self.outputs);
        if result == false {
            return false;
        }
        self.write_output_messages(bus);
        return true;
    }
}