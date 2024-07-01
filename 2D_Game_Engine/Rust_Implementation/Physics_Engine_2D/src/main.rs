use vect2::Vector2;

mod vect2;

fn main() {
    let v1 : Vector2 = Vector2::new(2.0,3.0);
    let v2 : Vector2 = Vector2::new(1.0,8.0);

    println!("{:?} + {:?} = {:?}",v1,v2,v1 + v2);
    println!("{:?} - {:?} = {:?}",v1,v2,v1 - v2);
    println!("{:?} * {:?} = {:?}",v1,v2,v1 * v2);
    println!("{:?} / {:?} = {:?}",v1,v2,v1 / v2);
    println!("{:?} * 2.0 = {:?}",v1,v1.scale(2.0));
    println!("{:?} . {:?} = {:?}",v1,v2,v1.dotProduct(v2));
    println!("{:?} perpendicular is {:?}",v1,v1.perpendicular());
    println!("{:?} Normalised is {:?}",v1,v1.normalise());
    println!("{:?} rotated 0.3 radians is {:?}",v1,v1.rotate(0.3));
}


