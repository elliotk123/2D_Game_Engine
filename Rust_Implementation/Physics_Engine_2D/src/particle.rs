use crate::Vector2;

#[derive(Debug, Copy, Clone)]
pub struct Particle
{
    pub position     : Vector2,
    linear_velocity  : Vector2,
    angular_velocity : f32,
    orientation      : f32
    
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

    pub fn update(&mut self, delta_t : f32, lin_acc : Vector2, ang_acc : f32)
    {
        self.orientation += self.angular_velocity * delta_t + ang_acc * 0.5 * delta_t.powf(2.0);
        self.angular_velocity += ang_acc * delta_t;
        self.position += self.linear_velocity * delta_t + lin_acc * 0.5 * delta_t.powf(2.0);
        self.linear_velocity += lin_acc * delta_t;
    }

}