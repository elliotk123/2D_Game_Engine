use super::game_logic_io::{GameLogicInputs, GameLogicOutputs};
use system_interface::common::keyboard_interface::{MyKeyboardEvent, MyKey};
use engine_common::engine_bus::{LogicToPhysicsChannel, LogicToRenderSyncChannel};

const SHAPE64 : [f64 ; 6] = [
    0.0, 1.0,
    -0.5, -0.5,
    0.5, -0.5
];

const SHAPE32 : [f32 ; 6] = [
    0.0, 1.0,
    -0.5, -0.5,
    0.5, -0.5
];

#[derive(Debug, Clone)]
pub struct GameLogicState {
    initialised : bool,
    thrust : f32,
    torque_clock : f32,
    torque_anti_clock : f32
}

impl GameLogicState {
    pub fn new() -> GameLogicState {
        GameLogicState { 
            initialised: false,
            thrust: 0.,
            torque_clock : 0.,
            torque_anti_clock : 0.
        }
    }
}

fn initialise(input : GameLogicInputs, state : &mut GameLogicState, output : & mut GameLogicOutputs) -> bool
{
    output.physics_commands.push(
        LogicToPhysicsChannel::AddEntity 
        { 
            mass: 1000.0, moi: 1000.0, 
            posx: 0.0, posy: 0.0, 
            velx: 0.0, vely: 0.0, 
            orien: 0.0, angvel: 0.0, 
            shape: SHAPE32.to_vec()
        }
    );
    output.render_sync_commands.push(
        LogicToRenderSyncChannel::DotGraphicsUpdate 
        { 
            index: 0, layer: 0.0, 
            dots: SHAPE64.to_vec() 
        }
    );
    true
}
pub fn process(input : GameLogicInputs, state : &mut GameLogicState, output : & mut GameLogicOutputs) -> bool
{
    if !state.initialised {
        if !initialise(input, state, output){
            return false;
        }
        state.initialised = true;
    }else{
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
        output.physics_commands.push(
            LogicToPhysicsChannel::ApplyCenterlineForce { index: 0, force: state.thrust}
        );
        output.physics_commands.push(
            LogicToPhysicsChannel::ApplyTorque 
            { 
                index: 0, 
                torque: state.torque_clock - state.torque_anti_clock
            }
        );
    }
    
    return true;
}

