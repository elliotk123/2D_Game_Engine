use std::collections::HashMap
use mod super

struct RenderSyncModule {
    camera: Camera,
    entities: HashMap<usize, RenderEntity>,
}

impl RenderSyncModule{
    pub fn new()->self
    {
        self
        {
            camera
        }
    }
}

impl EngineModule for RenderSyncModule{
    fn run(&mut self, bus : &mut EngineBus)
    {
        //1. Drain LogicToRenderSync channel
        //2. Drain PhysicsToRenderSync channel
        //3. For each entity:
            //a. Transform local dots → world space
            //b. Apply orientation
            //c. Apply camera offset
            //d. Convert to screen coordinates
            //e. Emit RenderCommand(s)