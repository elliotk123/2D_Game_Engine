use particle::Particle;
use vector2::Vector2;

mod particle;
mod vector2;



fn main() {
    let v1 : Vector2 = Vector2::new(2.0,3.0);
    let v2 : Vector2 = Vector2::new(1.0,8.0);

    let p1 : Particle = Particle::new(Vector2::new(1.0, 1.0), 
                                  Vector2::new(0.03, 0.04), 
                                  0.0, 0.0);
                                  let delta_t = 2.3;
    println!("{:?} + {:?} = {:?}",v1,v2,v1 + v2);
    println!("{:?} - {:?} = {:?}",v1,v2,v1 - v2);
    println!("{:?} * {:?} = {:?}",v1,v2,v1 * v2);
    println!("{:?} / {:?} = {:?}",v1,v2,v1 / v2);
    println!("{:?} * 2.0 = {:?}",v1,v1 * 2.0);
    println!("{:?} . {:?} = {:?}",v1,v2,v1.dot_product(v2));
    println!("{:?} perpendicular is {:?}",v1,v1.perpendicular());
    println!("{:?} Normalised is {:?}",v1,v1.normalise());
    println!("{:?} rotated 0.3 radians is {:?}",v1,v1.rotate(0.3));

    println!("{:?} moves to ",p1);
    p1.update(delta_t);
    println!("{:?} after {:?} seconds",p1.position,delta_t);
}


