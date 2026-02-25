use engine_common::engine_bus::ForceField;
use engine_math::vector2::Vector2;

use crate::entity::Entity;
use crate::force_field::{
        calculate_constant_force, 
        calculate_inverse_square
    };
use crate::particle_group::ParticleGroup;
use crate::shape::Shape;

pub struct EntityGroup {
    pub particles : ParticleGroup,
    shapes : Box<[Shape]>,
    pub masses  : Box<[f64]>,
    pub moments_of_intertia : Box<[f64]>,
    force_x_accumulators : Box<[f64]>,
    force_y_accumulators : Box<[f64]>,
    torque_accumulators : Box<[f64]>,
    num_entities : usize,
    pub num_active_entities : usize
}

impl EntityGroup{
    pub fn new(num_particles_pow_2 : u32) -> Self{
        let num_entities: usize = 2_usize.pow(num_particles_pow_2);
        Self{ 
            particles : ParticleGroup::new(num_particles_pow_2),
            shapes : vec![Shape::new(Vec::new()); num_entities].into_boxed_slice(),
            masses : vec![0.0; num_entities].into_boxed_slice(),
            moments_of_intertia : vec![0.0; num_entities].into_boxed_slice(),
            force_x_accumulators : vec![0.0; num_entities].into_boxed_slice(),
            force_y_accumulators : vec![0.0; num_entities].into_boxed_slice(),
            torque_accumulators : vec![0.0; num_entities].into_boxed_slice(),
            num_entities,
            num_active_entities : 0
        }
    }

    pub fn add_entity(&mut self, entity : Entity){
        if self.num_active_entities < self.num_entities {
            self.particles.add_particle(entity.particle);
            self.shapes[self.num_active_entities] = entity.shape;
            self.masses[self.num_active_entities] = entity.mass;
            self.moments_of_intertia[self.num_active_entities] = entity.moment_of_inertia;
            self.num_active_entities += 1;
        }else{
            println!("Cannot add entity, entit group full");
        }
    }

    pub fn apply_force(&mut self, force:Vector2, entity_index : usize)
    {
        if entity_index >= self.num_active_entities{
            println!("Cannot apply force to entity {}, entity not active", entity_index);
        }else{
            self.force_x_accumulators[entity_index] += force.x;
            self.force_y_accumulators[entity_index] += force.y;
        }
    }

    pub fn apply_centerline_force(&mut self, force:f64, entity_index : usize)
    {
        if entity_index >= self.num_active_entities{
            println!("Cannot apply centerline force to entity {}, enttiy not active", entity_index);
        }else{
            let force = 
            Vector2 { x: (0.0), y: (force) }
            .rotate(self.particles.orientations[entity_index]);
            self.force_x_accumulators[entity_index] += force.x;
            self.force_y_accumulators[entity_index] += force.y;
        }
    }


    pub fn apply_torque(&mut self, torque:f64, entity_index : usize)
    {
        if entity_index >= self.num_active_entities{
            println!("Cannot apply torque to entity {}, entity not active", entity_index);
        }else{
            self.torque_accumulators[entity_index] += torque;
        }
    }

    pub fn apply_field(&mut self, field : &ForceField){
        match field{
            ForceField::Constant 
            {
                force, charge 
            }  => {
                for((fx, fy),c) in 
                self.force_x_accumulators.iter_mut()
                .zip(&mut self.force_y_accumulators)
                .zip(charge)
                .take(self.num_active_entities){
                    let calculated_force = calculate_constant_force(force, *c);
                    *fx += calculated_force.x;
                    *fy += calculated_force.y;
                }
            },
            ForceField::InverseSquare 
            { 
                source, 
                constant, 
                charge 
            } => {
                let source_position = Vector2
                {
                    x : self.particles.positions_x[*source],
                    y : self.particles.positions_y[*source],
                };
                for(((p, fx),fy),c) in 
                self.particles.into_iter()
                .zip(&mut self.force_x_accumulators)
                .zip(&mut self.force_y_accumulators)
                .zip(charge)
                .take(self.num_active_entities){
                    let force = calculate_inverse_square
                    (
                        &source_position,
                        *constant, 
                        *c, 
                        &Vector2{x: *p.x, y : *p.y} 
                    );
                    *fx += force.x;
                    *fy += force.y;
                }
            }
            _ => {
            }
        }
    }

    pub fn update(&mut self, delta_t : f64){
        let n = self.num_active_entities;
        // iterate through forces and torques to calculate accelerations
        for((((fx,fy),m),t),i) in 
        self.force_x_accumulators.iter_mut()
        .zip(self.force_y_accumulators.iter_mut())
        .zip(&self.masses)
        .zip(self.torque_accumulators.iter_mut()).zip(&self.moments_of_intertia){
            *fx /= *m;
            *fy /= *m;
            *t /= *i;
        }

        // update particles velocity and position with calculated acceleration
        self.particles.update(
            0,
            delta_t,
             &self.force_x_accumulators[..n], 
             &self.force_y_accumulators[..n],
             &self.torque_accumulators[..n],
             
        );
        // reset accumulated force values
        self.force_x_accumulators.fill(0.0);
        self.force_y_accumulators.fill(0.0);
        self.torque_accumulators.fill(0.0);
    }

    pub fn delete_entity(&mut self, index : usize){
        if index >= self.num_active_entities{
            println!("Cannot remove entity {}, enttiy not active", index);
        }else if index == self.num_active_entities - 1{
            self.num_active_entities -= 1;
        }else{
            self.particles.delete_particle(index);
            self.shapes.swap(index,self.num_active_entities-1);
            self.masses[index] = self.masses[self.num_active_entities-1];
            self.moments_of_intertia[index] = self.moments_of_intertia[self.num_active_entities-1];
            self.force_x_accumulators[index] = self.force_x_accumulators[self.num_active_entities-1];
            self.force_y_accumulators[index] = self.force_y_accumulators[self.num_active_entities-1];
            self.torque_accumulators[index] = self.torque_accumulators[self.num_active_entities-1];
            self.num_active_entities -= 1;
        }
    }


}