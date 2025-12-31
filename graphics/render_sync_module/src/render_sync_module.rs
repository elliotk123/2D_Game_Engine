use std::collections::HashMap;
use crate::camera::Camera;
use crate::render_entity::RenderEntity;
use engine_common::
{
    engine_module::EngineModule,
    engine_bus::
    {
        EngineBus,
        RenderCommand,
        PhysicsToRenderSyncChannel,
        LogicToRenderSyncChannel
    },
};

pub struct RenderSyncModule {
    camera: Camera,
    entities: HashMap<usize, RenderEntity>,
}

impl RenderSyncModule{
    pub fn new()->Self {
        Self {
            camera: Camera {x:0.0,y:0.0},
            entities: HashMap::new(),
        }
    }
}

impl EngineModule for RenderSyncModule {
    fn run(&mut self, bus : &mut EngineBus)->bool {
        //1. Drain LogicToRenderSync channel
        while let Ok(msg) = bus.logic_to_render_sync.rx.try_recv() {
            match msg {
                LogicToRenderSyncChannel::Camera { posx, posy } => {
                    self.camera.x = posx;
                    self.camera.y = posy;
                }

                LogicToRenderSyncChannel::DotGraphicsUpdate { index, dots, layer} => {
                    let entity = self.entities.entry(index).or_insert_with(RenderEntity::default);
                    entity.dots = dots;
                    entity.layer_height = layer;
                }

                LogicToRenderSyncChannel::ChangeEntityColour { index, colour_id} => {
                    let entity = self.entities.entry(index).or_insert_with(RenderEntity::default);
                    entity.colour_id = colour_id;
                },
                LogicToRenderSyncChannel::BackgroundTexture => todo!()
            }
        }
        //2. Drain PhysicsToRenderSync channel
        while let Ok(msg) = bus.physics_to_render_sync.rx.try_recv() {
            match msg {
                PhysicsToRenderSyncChannel::Position { index, x, y } => {
                    let entity = self.entities.entry(index).or_insert_with(RenderEntity::default);
                    entity.x = x;
                    entity.y = y;
                }

                PhysicsToRenderSyncChannel::Orientation { index, orientation } => {
                    let entity = self.entities.entry(index).or_insert_with(RenderEntity::default);
                    entity.orientation = orientation;
                }
            }
        }
        //3. For each entity:
            //a. Transform local dots → world space
            //b. Apply orientation
            //c. Apply camera offset
            //d. Convert to screen coordinates
            //e. Emit RenderCommand(s)
        for entity in self.entities.values() {
            if entity.dots.is_empty() {
                continue;
            }
            let angle = entity.orientation;
            let cos_a = angle.cos();
            let sin_a = angle.sin();
            let colour = entity.colour_id;
            //THIS ALL NEEDS REFACTORING TO USE Vector2 STRUCT
            for chunk in entity.dots.chunks_exact(2) {
                let local_x = chunk[0];
                let local_y = chunk[1];

                // Rotate local point
                let rotated_x = local_x * cos_a - local_y * sin_a;
                let rotated_y = local_x * sin_a + local_y * cos_a;

                // Rotated Local -> world
                let world_x = entity.x + rotated_x;
                let world_y = entity.y + rotated_y;

                // World -> camera space
                let camera_x = world_x - self.camera.x;
                let camera_y = world_y - self.camera.y;

                // Camera space -> screen
                let screen_x = camera_x as i32;
                let screen_y = camera_y as i32;

                bus.render_sync_to_compositor.tx.send(
                RenderCommand::Pixel {
                    x: screen_x,
                    y: screen_y,
                    colour_id: colour}).unwrap();
            }
        }
        return true;
    }
}