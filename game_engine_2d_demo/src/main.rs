use std::time::Duration;

use engine_core::run;
use engine_common::{
    engine_module::ModuleId,
    engine_bus::{LogicToPhysicsChannel, LogicToRenderSyncChannel},
    game_conf::{GameLogicInputs, GameLogicOutputs,GameConf}
};
use system_interface::common::keyboard_interface::{MyKeyboardEvent, MyKey};

const SHAPE64 : [f64 ; 6] = [
    0.0, 10.0,
    -5.0, -5.0,
    5.0, -5.0
];

const SHAPE32 : [f32 ; 6] = [
    0.0, 10.0,
    -5.0, -5.0,
    5.0, -5.0
];

#[derive(Debug, Clone)]
pub struct GameConfig {
    initialised : bool,
    thrust : f32,
    torque_clock : f32,
    torque_anti_clock : f32
}

impl GameConfig {
    pub fn initialise(output : &mut GameLogicOutputs) -> bool
    {
        output.physics_commands.push(
            LogicToPhysicsChannel::AddEntity 
            { 
                mass: 1000.0, moi: 1000.0, 
                posx: 100.0, posy: 100.0, 
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
        output.render_sync_commands.push(
            LogicToRenderSyncChannel::ChangeEntityColour { index: 0, colour_id: 12 }
        );
        true
    }
}

impl GameConf for GameConfig {
    fn new() -> GameConfig {
        GameConfig { 
            initialised: false,
            thrust: 0.,
            torque_clock : 0.,
            torque_anti_clock : 0.
        }
    }

    fn process(&mut self, input: &GameLogicInputs, output : &mut GameLogicOutputs) -> bool
    {      
        if !self.initialised {
            if !GameConfig::initialise(output){
                return false;
            }
            self.initialised = true;
        }else{
            for event in &input.keyboard_events
            {
                match event
                {
                    MyKeyboardEvent::KeyDown(key)=>
                    {
                        match key {
                            MyKey::Escape => return false,
                            MyKey::Space => self.thrust = 50000.0,
                            MyKey::A => self.torque_anti_clock = 10000.0,
                            MyKey::D => self.torque_clock = 10000.0,
                            _ => {}                    
                        }
                    },
                    MyKeyboardEvent::KeyUp(key)=>
                    {
                        match key {
                            MyKey::Space => self.thrust = 0.0,
                            MyKey::A => self.torque_anti_clock = 0.0,
                            MyKey::D => self.torque_clock = 0.0,
                            _ => {}    
                        }
                    
                    }
                }
            }
            if self.thrust != 0.0 {
                output.physics_commands.push(
                    LogicToPhysicsChannel::ApplyCenterlineForce { index: 0, force: self.thrust}
                );
            }

            output.physics_commands.push(
                LogicToPhysicsChannel::ApplyTorque 
                { 
                    index: 0, 
                    torque: self.torque_clock - self.torque_anti_clock
                }
            );
        }
        
        return true;
    }
}


fn main() {
    let schedule: Vec<Vec<ModuleId>> = vec![
        vec![
            ModuleId::SYSIN,
            ModuleId::LOGIC, 
            ModuleId::PHYS2D, 
            ModuleId::RENDER,
            ModuleId::COMP2D, 
            ModuleId::SYSOUT,
        ]
    ];

    let minor_cycle : Duration = Duration::from_nanos(16666667);
    run::<GameConfig>(schedule, minor_cycle);
}
