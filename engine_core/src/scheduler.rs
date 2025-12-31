use std::time::{Duration, Instant};

use engine_common::engine_module::ModuleId;
use super::game_state::GameState;

pub struct Scheduler {
    minor_cycle_dur : Duration, // The minor cycle of the scheduler
    schedule : Vec<Vec<ModuleId>>

}

impl Scheduler{
    pub fn new(minor_cycle_us: u64, schedule : Vec<Vec<ModuleId>>) -> Scheduler{
        Scheduler{
            minor_cycle_dur : Duration::from_micros(minor_cycle_us),
            schedule : schedule.clone()
        }
    }

    pub fn run(&self, game : &mut GameState)->bool{
        for minor_cycle in self.schedule.iter()
        {
            let minor_cycle_start_time = Instant::now(); 
            for task in minor_cycle.iter()
            {
                let task_start_time : Instant = Instant::now();
                let result = game.run(task.clone());
                if result == false {
                    return false;
                }
                let task_duration = task_start_time.elapsed();
                println!("Task {} us", task_duration.as_micros());

            }
            let minor_cycle_duration : Duration = minor_cycle_start_time.elapsed();
            if minor_cycle_duration > self.minor_cycle_dur
            {
                println!("OVERFRAME! {} us", minor_cycle_duration.as_micros());
                return false;
            }
            else
            {
                loop
                {
                    if minor_cycle_start_time.elapsed() > self.minor_cycle_dur
                    {
                        break;
                    };
                }
            }
        }
        return true;
    }
}