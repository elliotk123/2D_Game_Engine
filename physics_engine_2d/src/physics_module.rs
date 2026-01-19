use super::entity::Entity;
use super::particle::Particle;
use super::shape::Shape;
use engine_math::vector2::Vector2;

use engine_common::{
    engine_bus::{
        EngineBus, 
        LogicToPhysicsChannel,
        PhysicsToRenderSyncChannel
    },
    engine_module::EngineModule
};

pub struct PhysicsModule {
    entities : Vec<Entity>,
    delta_t_s : f32,
}

impl PhysicsModule
{
    pub fn new(delta_t : f32, )->PhysicsModule
    {
        PhysicsModule
        {
            entities : Vec::new(),
            delta_t_s : delta_t
        }

    }

    fn handle_messages(&mut self, bus :  &mut EngineBus){
        while let Ok(msg) = bus.logic_to_physics.rx.try_recv(){
            match msg{
                LogicToPhysicsChannel::AddEntity{
                    mass,
                    moi,
                    posx,
                    posy,
                    velx,
                    vely,
                    orien,
                    angvel,
                    shape
                } => {
                    self.entities.push(Entity::new(
                        Particle::new(
                            Vector2 { x: posx, y: posy },
                            Vector2 { x: velx, y: vely },
                            angvel,
                            orien
                        ),
                        Shape::new(
                            shape
                            .chunks(2)
                            .filter(|chunk| chunk.len() == 2)
                            .map(|chunk| Vector2{
                                x: chunk[0],
                                y: chunk[1]
                            })
                            .collect()
                        ),
                        mass,
                        moi
                    ));
                },
                LogicToPhysicsChannel::RemoveEntity{
                    index
                } => {
                    self.entities.remove(index);
                },
                LogicToPhysicsChannel::ApplyForce { 
                    index, forcex, forcey 
                } => {
                    self.entities[index].apply_force(
                        Vector2 {
                             x: forcex, 
                             y: forcey 
                        }
                    );
                },
                LogicToPhysicsChannel::ApplyCenterlineForce { 
                    index, force 
                } => {
                    self.entities[index].apply_centerline_force(force);
                    // println!("APPLY CENTERLINE FORCE {} {}", index, force);
                }
                LogicToPhysicsChannel::ApplyTorque { 
                    index, 
                    torque 
                } => {
                    self.entities[index].apply_torque(torque);
                }
            }
        }
    }
}

impl EngineModule for PhysicsModule
{
    fn run(&mut self, bus : &mut EngineBus)->bool
    {
        self.handle_messages(bus);

        for entity in self.entities.iter_mut()
        {
            entity.update(self.delta_t_s);
        }

        let i = 0;

        for entity in self.entities.iter_mut()
        {
            bus.physics_to_render_sync.tx.send(PhysicsToRenderSyncChannel::Position{
                index : i,
                x : entity.particle.position.x as f64,
                y : entity.particle.position.y as f64,
            }).unwrap();
            bus.physics_to_render_sync.tx.send(PhysicsToRenderSyncChannel::Orientation {
                index : i,
                orientation : entity.particle.orientation as f64,
            }).unwrap();
        }
        return true;
    }
}