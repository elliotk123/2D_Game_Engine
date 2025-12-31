use engine_core::{scheduler::Scheduler, game_state::GameState};
use engine_common::engine_module::ModuleId;

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

    let minor_cycle : u64 = 16667;
    let scheduler = Scheduler::new(minor_cycle, schedule);

    let mut game_state : GameState = GameState::new(minor_cycle as f32/1000000.0);

    let mut run = true;

    while run == true{
        run = scheduler.run(&mut game_state);
    }

}
