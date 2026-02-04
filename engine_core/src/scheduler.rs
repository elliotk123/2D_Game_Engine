use std::time::{Duration, Instant};

use engine_common::{
    engine_module::ModuleId,
    game_conf::GameConf
};
use super::game_state::GameState;

pub struct Scheduler {
    schedule : Vec<Vec<ModuleId>>

}

impl Scheduler{
    pub fn new(schedule : Vec<Vec<ModuleId>>) -> Scheduler{
        Scheduler{
            schedule : schedule.clone()
        }
    }

    pub fn run<T : GameConf>(&self, game : &mut GameState<T>, minor_cycle_dur: Duration)->bool{
        for minor_cycle in self.schedule.iter()
        {
            let minor_cycle_start_time = Instant::now(); 
            for task in minor_cycle.iter()
            {
                // let task_start_time : Instant = Instant::now();
                let result = game.run(task.clone());
                if result == false {
                    return false;
                }
                // let task_duration = task_start_time.elapsed();
                // println!("Task {} us", task_duration.as_micros());

            }
            // println!("\n");
            let minor_cycle_duration : Duration = minor_cycle_start_time.elapsed();
            // println!("Total process time {} ms", minor_cycle_duration.as_millis());
            if minor_cycle_duration > minor_cycle_dur
            {
                println!("OVERFRAME! {} us", minor_cycle_duration.as_micros());
                // return false;
            }
            else
            {
                loop
                {
                    if minor_cycle_start_time.elapsed() > minor_cycle_dur
                    {
                        break;
                    };
                }
            }
        }
        return true;
    }
}