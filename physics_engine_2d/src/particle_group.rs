use crate::particle::Particle;


pub struct ParticleGroup
{
    num_particles : usize,
    num_active_particles : usize,
    pub positions_x : Box<[f32]>,
    pub positions_y : Box<[f32]>,
    velocities_x : Box<[f32]>,
    velocities_y : Box<[f32]>,
    pub orientations : Box<[f32]>,
    angular_vels : Box<[f32]>
}

impl ParticleGroup{
    pub fn new(num_particles_pow_2 : u32) -> Self{
        let num_particles: usize = 2_usize.pow(num_particles_pow_2);
        Self{ 
            num_particles,
            num_active_particles : 0,
            positions_x : vec![0.0; num_particles].into_boxed_slice(),
            positions_y : vec![0.0; num_particles].into_boxed_slice(),
            velocities_x : vec![0.0; num_particles].into_boxed_slice(),
            velocities_y : vec![0.0; num_particles].into_boxed_slice(),
            orientations : vec![0.0; num_particles].into_boxed_slice(),
            angular_vels : vec![0.0; num_particles].into_boxed_slice(),
        }
    }

    pub fn update(
        &mut self,
        delta_t : f32, 
        accel_x : &[f32], 
        accel_y : &[f32], 
        accel_ang : &[f32]
    ){
        update_dof(&mut self.positions_x,&mut self.velocities_x,&accel_x, delta_t);
        update_dof(&mut self.positions_y,&mut self.velocities_y,&accel_y, delta_t);
        update_dof(&mut self.orientations,&mut self.angular_vels,&accel_ang, delta_t);
    }

    pub fn add_particle(&mut self, particle : Particle){
        if self.num_active_particles < self.num_particles {
            self.positions_x[self.num_active_particles] = particle.position.x;
            self.positions_y[self.num_active_particles] = particle.position.y;
            self.velocities_x[self.num_active_particles] = particle.linear_velocity.x;
            self.velocities_y[self.num_active_particles] = particle.linear_velocity.y;
            self.orientations[self.num_active_particles] = particle.orientation;
            self.angular_vels[self.num_active_particles] = particle.angular_velocity;
            self.num_active_particles += 1;
        }else{
            println!("CAnnot add particle, particle group full");
        }
    }

    pub fn delete_particle(&mut self, particle_index : usize){
        if particle_index >= self.num_active_particles{
            println!("Cannot remove particle {}, particle not active", particle_index);
        }else if particle_index == self.num_active_particles - 1{
            self.num_active_particles -= 1;
        }else{
            self.positions_x[particle_index] = self.positions_x[self.num_active_particles-1];
            self.positions_y[particle_index] = self.positions_y[self.num_active_particles-1];
            self.velocities_x[particle_index] = self.velocities_x[self.num_active_particles-1];
            self.velocities_y[particle_index] = self.velocities_y[self.num_active_particles-1];
            self.orientations[particle_index] = self.orientations[self.num_active_particles-1];
            self.angular_vels[particle_index] = self.angular_vels[self.num_active_particles-1];
            self.num_active_particles -= 1;
        }
    }
}

fn update_dof(p : &mut [f32], v : &mut [f32], a : &[f32], delta_t : f32)
{
    for((pos, vel), acc) in 
        p.iter_mut().zip(v.iter_mut()).zip(a.iter())
    {
        *pos += delta_t * (*vel + delta_t* *acc/2.0);
        *vel += delta_t * *acc;
    }
}

pub struct ParticleRef<'a> {
    pub x: &'a f32,
    pub y: &'a f32,
    pub vx: &'a f32,
    pub vy: &'a f32,
    pub orientation: &'a f32,
    pub ang_vel: &'a f32,
}

impl<'a> IntoIterator for &'a ParticleGroup {
    type Item = ParticleRef<'a>;
    type IntoIter = Box<dyn Iterator<Item = Self::Item> + 'a>;

    fn into_iter(self) -> Self::IntoIter {
        let iter = self.positions_x.iter()
            .zip(self.positions_y.iter())
            .zip(self.velocities_x.iter())
            .zip(self.velocities_y.iter())
            .zip(self.orientations.iter())
            .zip(self.angular_vels.iter())
            .take(self.num_active_particles)
            .map(|(((((x, y), vx), vy), o), av)| ParticleRef {
                x, y, vx, vy, orientation: o, ang_vel: av
            });
        
        Box::new(iter)
    }
}