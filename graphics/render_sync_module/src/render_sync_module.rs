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
            camera: Camera::new(),
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

                LogicToRenderSyncChannel::ChangeEntitySprite { index, sprite_key} => {
                    let entity = self.entities.entry(index).or_insert_with(RenderEntity::default);
                    entity.sprite_key = sprite_key;
                },

                LogicToRenderSyncChannel::ChangeEntityColor { index, colour_id } => {
                    let entity = self.entities.entry(index).or_insert_with(RenderEntity::default);
                    entity.colour_id = colour_id;
                }
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
            if entity.sprite_key == "N_A" {
                continue;
            }
            let angle = entity.orientation;

            //THIS ALL NEEDS REFACTORING TO USE Vector2 STRUCT
            for entity in self.entities.values() {

                
                // Skip entities that don't have a sprite set yet

                if entity.sprite_key.is_empty() || 
                   entity.sprite_key == "N_A" {
                        continue;
                    }

                // World -> camera space
                let camera_x = entity.x - self.camera.x;
                let camera_y = entity.y - self.camera.y;

                // Camera space -> screen
                let screen_x = (camera_x * self.camera.ppm) as i32;
                let screen_y = (camera_y * self.camera.ppm) as i32;

                // Debug
                // println!("RenderSync sprite '{}' at {},{}", entity.sprite_key, screen_x, screen_y);

                bus.render_sync_to_compositor.tx
                    .send(RenderCommand::Sprite {
                        x: screen_x,
                        y: screen_y,
                        sprite_key: entity.sprite_key.clone(),
                    })
                    .unwrap();

                if  entity.dots.len() > 0 {
                    for chunk in entity.dots.chunks_exact(2)
                    {
                        println!("Draw a pixel({},{})",chunk[0],chunk[1]);
                        let local_x = chunk[0];
                        let local_y = chunk[1];

                        let world_x = entity.x + local_x;
                        let world_y = entity.y + local_y;

                        // World -> camera space
                        let camera_x = world_x - self.camera.x;
                        let camera_y = world_y - self.camera.y;

                        // Camera space -> screen
                        let screen_x = (camera_x * self.camera.ppm) as i32;
                        let screen_y = (camera_y * self.camera.ppm) as i32;
                        
                        bus.render_sync_to_compositor.tx
                            .send(RenderCommand::Pixel{ x: screen_x, y: screen_y, colour_id: entity.colour_id}).unwrap();
                    }
                }            
            }
        }
        return true;
    }
}