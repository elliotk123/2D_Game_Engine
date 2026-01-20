use engine_math::vector2::Vector2;
use crate::particle::{Particle};
use crate::shape::{Shape};

#[derive(Debug, Clone)]
pub struct Entity
{
    pub particle       : Particle,
    pub shape          : Shape,
    mass               : f32,
    moment_of_inertia  : f32,
    force_accumulator  : Vec<Vector2>,
    torque_accumulator : Vec<f32>
}
impl Entity
{
    pub fn new (particle: Particle, shape:Shape,mass:f32,moment_of_inertia : f32) -> Entity
    {
        Entity
        {
            particle,
            shape,
            mass,
            moment_of_inertia,
            force_accumulator : Vec::new(),
            torque_accumulator : Vec::new()
        }
    }
    pub fn apply_force(&mut self, force:Vector2)
    {
        self.force_accumulator.push(force);
    }

    pub fn apply_centerline_force(&mut self, force: f32)
    {
        self.force_accumulator.push(
            Vector2 { x: (0.0), y: (force) }.rotate(self.particle.orientation)
        )
    }


    pub fn apply_torque( &mut self, torque: f32)
    {
        self.torque_accumulator.push(torque);
    }

    pub fn update( &mut self, delta_t:f32)
    {
        let mut total_force = Vector2::new(0.0, 0.0);
        while self.force_accumulator.len() != 0
        {
            total_force += self.force_accumulator.pop().unwrap();
        }

        let mut total_torque = 0.0;
        while self.torque_accumulator.len() != 0 
        {
            total_torque += self.torque_accumulator.pop().unwrap();
        }

        self.particle.update(delta_t, total_force.scale(1.0/self.mass), total_torque/self.moment_of_inertia);
    }

}