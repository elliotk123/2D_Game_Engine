use crate::Vector2;

#[derive(Debug, Copy, Clone)]
pub struct Particle
{
    pub position         : Vector2,
    linear_velocity  : Vector2,
    angular_velocity : f32,
    orientation      : f32,
    
}
impl Particle
{
    pub fn new(pos : Vector2, lin_vel: Vector2, angular_vel : f32, angle : f32) -> Particle
    {
        Particle
        {
            position         : pos,
            linear_velocity  : lin_vel,
            angular_velocity : angular_vel,
            orientation      : angle,
        }
    }

    pub fn update(mut self, delta_t : f32)
    {
        self.position += self.linear_velocity * delta_t;
    }
}