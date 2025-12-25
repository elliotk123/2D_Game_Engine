use super::game_logic_io::{GameLogicInputs, GameLogicOutputs};
use system_interface::common::keyboard_interface::{MyKeyboardEvent, MyKey};
pub struct GameLogicState {
    thrust : f64,
    torque_clock : f64,
    torque_anti_clock : f64
}

impl GameLogicState {
    pub fn new() -> GameLogicState {
        GameLogicState { 
            thrust: 0.,
            torque_clock : 0.,
            torque_anti_clock : 0.
        }
    }
}

pub fn process(input : GameLogicInputs, state : &mut GameLogicState, output : & mut GameLogicOutputs) -> bool
{
    for event in input.keyboard_events
    {
        match event
        {
            MyKeyboardEvent::KeyDown(key)=>
            {
                match key {
                    MyKey::Escape => return false,
                    MyKey::Space => state.thrust = 10.0,
                    MyKey::A => state.torque_anti_clock = 10.0,
                    MyKey::D => state.torque_clock = 10.0,
                    _ => {}                    
                }
            },
            MyKeyboardEvent::KeyUp(key)=>
            {
                match key {
                    MyKey::Space => state.thrust = 0.0,
                    MyKey::A => state.torque_anti_clock = 0.0,
                    MyKey::D => state.torque_clock = 0.0,
                    _ => {}    
                }
            
            }
        }
    }
    return true;
}

