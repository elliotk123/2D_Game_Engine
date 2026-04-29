use std::time::Duration;

use crate::{entity_group::EntityGroup, force_field::reconfigure_deleted_entity};

use super::entity::Entity;
use super::particle::Particle;
use super::shape::Shape;
use engine_math::vector2::Vector2;

use engine_common::{
    engine_bus::{
        EngineBus, 
        LogicToPhysicsChannel,
        PhysicsStateToLogicChannel,
        PhysicsToRenderSyncChannel,
        ForceField
    },
    engine_module::EngineModule
};

pub struct PhysicsModule {
    entity_groups : Vec<EntityGroup>,
    force_fields : Vec<Vec<ForceField>>,
    delta_t_s : f64,
}

impl PhysicsModule
{
    pub fn new(delta_t : Duration, )->PhysicsModule
    {
        PhysicsModule
        {
            entity_groups : Vec::new(),
            force_fields : Vec::new(),
            delta_t_s : delta_t.as_secs_f64()
        }

    }

    fn handle_messages(&mut self, bus :  &mut EngineBus){
        while let Ok(msg) = bus.logic_to_physics.rx.try_recv(){
            match msg{
                LogicToPhysicsChannel::AddEntityGroup {
                     num_entities_pow_2 
                }=>{
                    self.entity_groups.push(EntityGroup::new(num_entities_pow_2));
                    self.force_fields.push(Vec::new());
                },
                LogicToPhysicsChannel::RemoveEntityGroup {
                    group_index,
                }=>{
                    if group_index < self.entity_groups.len(){
                        self.entity_groups.swap_remove(group_index);
                         self.force_fields.swap_remove(group_index);
                    }else{
                        println!("Cannot remove entity group {} as it doesn't exist", group_index);
                    }
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
                    for field in self.force_fields[group_index].iter_mut(){
                        reconfigure_deleted_entity(index, field);
                    }
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
                    // self.entity_groups[group_index].apply_field(field);
                    self.force_fields[group_index].push(field);
                },
                LogicToPhysicsChannel::DeleteField {
                    group_index,
                    field_index
                }=>{
                     if let Some(group_fields) = self.force_fields.get_mut(group_index){
                        if field_index < group_fields.len() {
                            self.force_fields[group_index].swap_remove(field_index);
                        }else{
                            println!("Field {} does not exist for group {}. cannot remove", field_index, group_index);
                        }
                    }else{
                        println!("Group {} does not exist, cannot delete an associated field", group_index);
                    };
                        
                },
                LogicToPhysicsChannel::UpdateField {
                    group_index,
                    field_index,
                    field
                }=>{
                    if let Some(group_fields) = self.force_fields.get_mut(group_index){
                        if let Some(old_field) = group_fields.get_mut(field_index){
                            *old_field = field;
                        }else{
                            println!("Field {} does not exist for group {}. cannot update", field_index, group_index);
                        }
                    }else{
                        println!("Group {} does not exist, cannot update an associated field", group_index);
                    };
                }
            }
        }
    }
}

impl EngineModule for PhysicsModule
{
    fn run(&mut self, bus : &mut EngineBus)->bool{
        self.handle_messages(bus);

        for (entity_group, fields) in self.entity_groups.iter_mut().zip(&self.force_fields)
        {
            for field in fields.iter(){
                entity_group.apply_field(field);
            }

            entity_group.update(self.delta_t_s);
        }

        let mut i = 0;

        for entity_group in self.entity_groups.iter_mut()
        {
            for particle in entity_group.particles.into_iter(){
                println!("Physics is sending position to render Sync: ({},{})",particle.x,particle.y);
                bus.physics_to_render_sync.tx.send(PhysicsToRenderSyncChannel::Position{
                    index : i,
                    x : *(particle.x),
                    y : *(particle.y),
                }).unwrap();
                bus.physics_to_render_sync.tx.send(PhysicsToRenderSyncChannel::Orientation {
                    index : i,
                    orientation : *(particle.orientation),
                }).unwrap();

                // NEW: physics state -> logic
                bus.physics_state_to_logic.tx.send(PhysicsStateToLogicChannel::EntityState {
                    index: i,
                    posx: *particle.x,
                    posy: *particle.y,
                    orientation: *particle.orientation,
                }).unwrap();
                i+=1;
            }
        }
        return true;
    }
}