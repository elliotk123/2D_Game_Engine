use std::time::Duration;

use crate::entity_group::EntityGroup;

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
    entity_groups : Vec<EntityGroup>,
    delta_t_s : f32,
}

impl PhysicsModule
{
    pub fn new(delta_t : Duration, )->PhysicsModule
    {
        PhysicsModule
        {
            entity_groups : Vec::new(),
            delta_t_s : delta_t.as_secs_f32()
        }

    }

    fn handle_messages(&mut self, bus :  &mut EngineBus){
        while let Ok(msg) = bus.logic_to_physics.rx.try_recv(){
            match msg{
                LogicToPhysicsChannel::AddEntityGroup {
                     num_entities_pow_2 
                }=>{
                    self.entity_groups.push(EntityGroup::new(num_entities_pow_2));
                }
                LogicToPhysicsChannel::AddEntity{
                    group_index,
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
                    self.entity_groups[group_index].add_entity(Entity::new(
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
                    group_index,
                    index
                } => {
                    self.entity_groups[group_index].delete_entity(index);
                },
                LogicToPhysicsChannel::ApplyForce { 
                    group_index,
                    index,
                    forcex,
                    forcey
                } => {
                    self.entity_groups[group_index].apply_force(
                        Vector2 {
                             x: forcex, 
                             y: forcey 
                        },
                        index
                    );
                },
                LogicToPhysicsChannel::ApplyCenterlineForce { 
                    group_index,
                    index,
                    force 
                } => {
                    self.entity_groups[group_index].apply_centerline_force(force, index);
                    // println!("APPLY CENTERLINE FORCE {} {}", index, force);
                },
                LogicToPhysicsChannel::ApplyTorque { 
                    group_index,
                    index, 
                    torque 
                } => {
                    self.entity_groups[group_index].apply_torque(torque, index);
                },
                LogicToPhysicsChannel::ApplyField {
                    group_index,
                    field
                }=> {
                    self.entity_groups[group_index].apply_field(field);
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

        for entity_group in self.entity_groups.iter_mut()
        {
            entity_group.update(self.delta_t_s);
        }

        let i = 0;

        for entity_group in self.entity_groups.iter_mut()
        {
            for particle in entity_group.particles.into_iter(){
                bus.physics_to_render_sync.tx.send(PhysicsToRenderSyncChannel::Position{
                    index : i,
                    x : *(particle.x) as f64,
                    y : *(particle.y) as f64,
                }).unwrap();
                bus.physics_to_render_sync.tx.send(PhysicsToRenderSyncChannel::Orientation {
                    index : i,
                    orientation : *(particle.orientation) as f64,
                }).unwrap();
            }
        }
        return true;
    }
}