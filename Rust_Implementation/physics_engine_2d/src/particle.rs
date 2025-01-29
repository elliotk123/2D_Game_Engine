use crate::vector2::Vector2;

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
#[cfg(test)]
mod tests 
{
    use super::*;

    #[test]
    fn test_particle_new()
    {
        let p_test : Particle = Particle::new(Vector2::new(2.0,3.0), Vector2::new(0.5, 0.25), 1.5, 0.5);
        assert_eq!(p_test.position,         Vector2::new(2.0, 3.0));
        assert_eq!(p_test.linear_velocity,  Vector2::new(0.5, 0.25)); 
        assert_eq!(p_test.angular_velocity, 1.5);
        assert_eq!(p_test.orientation,      0.5);
    }

    #[test]
    fn test_particle_update()
    {
        let mut p_test  : Particle = Particle::new(Vector2::new(0.0,0.0), Vector2::new(2.0, 3.0), 0.0, 0.0);
        let mut delta_t : f32 = 1.0;
        
        p_test.update(delta_t, Vector2::new(1.0,1.0), 0.0);
        
        assert_eq!(p_test.position,         Vector2::new(2.5,3.5));
        assert_eq!(p_test.linear_velocity,  Vector2::new(3.0, 4.0));
        assert_eq!(p_test.angular_velocity, 0.0);
        assert_eq!(p_test.orientation,      0.0);

        p_test = Particle::new(Vector2::new(0.0,0.0), Vector2::new(0.0, 0.0), 1.0, 1.0);
        delta_t = 3.0;

        p_test.update(delta_t, Vector2::new(0.0,0.0), 1.0);
        assert_eq!(p_test.position,         Vector2::new(0.0,0.0));
        assert_eq!(p_test.linear_velocity,  Vector2::new(0.0,0.0));
        assert_eq!(p_test.angular_velocity, 4.0);
        assert_eq!(p_test.orientation,      8.5);

    }
}