use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;
use std::ops::SubAssign;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector2
{
    pub x  : f32,
    pub y  : f32
}
impl Vector2
{
    pub fn new(x_val : f32, y_val : f32) -> Vector2
    {
        Vector2
        {
            x : x_val,
            y : y_val
        }
    }
    pub fn magnitude(self) -> f32
    {
        return (self.x.powf(2.0) + self.y.powf(2.0)).sqrt();
    }
    pub fn scale(self, value : f32) -> Vector2
    {
        Vector2
        {
            x : self.x * value,
            y : self.y * value
        }
    }
    pub fn dot_product(self, rhs : Vector2) -> f32
    {
        return (self.x * rhs.x) + (self.y * rhs.y);
    }
    pub fn perpendicular(self) -> Vector2
    {
        Vector2
        {
            x : self.y,
            y : -self.x 
        }
    }
    pub fn normalise(self) -> Vector2
    {
        let length : f32 = self.magnitude();
        return  Vector2
                {
                    x : self.x / length,
                    y : self.y / length
                }
    }
    pub fn rotate(self, theta: f32) -> Vector2
    {
        Vector2
        {
            x : self.x * theta.cos() - self.y * theta.sin(),
            y : self.x * theta.sin() + self.y * theta.cos()
        }
    }
}

impl Sub for Vector2
{
    type Output = Vector2;
    fn sub(self, _rhs : Vector2) -> Vector2
    {
        Vector2
        {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y
        }
    }
}

impl Add for Vector2
{
    type Output = Vector2;
    fn add(self, _rhs: Vector2) -> Vector2
    {
        Vector2
        {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y
        }
    }
}

impl AddAssign<Vector2> for Vector2
{ 
    fn add_assign(&mut self, rhs: Vector2) 
    {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl SubAssign<Vector2> for Vector2
{
    fn sub_assign(&mut self, rhs: Vector2) 
    {
        self.x -= rhs.x;
        self.y -= rhs.y; 
    }
}

impl Mul<Vector2> for Vector2
{
    type Output = Vector2;
    fn mul(self, _rhs: Vector2) -> Vector2
    {
        Vector2
        {
            x: self.x * _rhs.x,
            y: self.y * _rhs.y
        }
    }
}

impl Mul<f32> for Vector2
{
    type Output = Vector2;
    fn mul(self,_rhs : f32) -> Vector2
    {
        Vector2{x: self.x,y: self.y}.scale(_rhs)
    }
}

impl Mul<i32> for Vector2
{
    type Output = Vector2;
    fn mul(self,_rhs : i32) -> Vector2
    {
        Vector2{x: self.x,y: self.y}.scale(_rhs as f32)
    }
}

impl Div for Vector2
{
    type Output = Vector2;
    fn div(self, _rhs: Vector2) -> Vector2
    {
        Vector2
        {
            x: self.x / _rhs.x,
            y: self.y / _rhs.y
        }
    }
}

#[cfg(test)]
mod tests 
{
    use super::*;

    #[test]
    fn test_vector_new () {
        let v_test : Vector2 = Vector2::new(2.0,3.0);
        assert_eq!(v_test.x, 2.0);
        assert_eq!(v_test.y, 3.0);
    }
    #[test]
    fn test_vector_addition () {
        let v1 : Vector2 = Vector2::new(2.0,3.0);
        let v2 : Vector2 = Vector2::new(1.0,8.0);
        let v_comp : Vector2 = Vector2::new(1.0,-5.0);
        let v_test = v1 - v2;
        assert_eq!(v_test, v_comp);
    }
    #[test]
    fn test_vector_addition_assign() {
        let mut v1 : Vector2 = Vector2::new(2.0,3.0);
        let v2 : Vector2 = Vector2::new(1.0,8.0);
        v1 += v2;
        let v_comp : Vector2 = Vector2::new(3.0,11.0);
        assert_eq!(v1, v_comp);
    }
    #[test] 
    fn test_vector_subtraction () {
        let v1 : Vector2 = Vector2::new(2.0,3.0);
        let v2 : Vector2 = Vector2::new(1.0,8.0);
        let v_comp : Vector2 = Vector2::new(1.0,-5.0);
        let v_test = v1 - v2;
        assert_eq!(v_test, v_comp);
    } 
    #[test]
    fn test_vector_subtraction_assign() {
        let mut v1 : Vector2 = Vector2::new(2.0,3.0);
        let v2 : Vector2 = Vector2::new(1.0,8.0);
        v1 -= v2;
        let v_comp : Vector2 = Vector2::new(1.0,-5.0);
        assert_eq!(v1, v_comp);
    }
    #[test]
    fn test_vector_multiplication () {
        let v1     : Vector2 = Vector2::new(2.0,3.0);
        let v2     : Vector2 = Vector2::new(1.0,8.0);
        let v_comp : Vector2 = Vector2::new(2.0,24.0);
        let v_test : Vector2 = v1 * v2;
        assert_eq!(v_test, v_comp);

        let i      : i32 = 3;
        let v_comp : Vector2 = Vector2::new(6.0,9.0);
        let v_test : Vector2 = v1 * i;
        assert_eq!(v_test, v_comp);

        let f      : f32 = 3.141; 
        let v_comp : Vector2 = Vector2::new(6.282, 9.423);
        let v_test : Vector2 = v1 * f;
        assert_eq!(v_test,v_comp);
    }
    #[test]
    fn test_vector_division () {
        let v1 : Vector2 = Vector2::new(7.0,3.0);
        let v2 : Vector2 = Vector2::new(2.0,8.0);
        let v_comp : Vector2 = Vector2::new(3.5,0.375);
        let v_test = v1 / v2;
        assert_eq!(v_test, v_comp);
    }
    #[test]
    fn test_vector_magnitude() {
        let v1 : Vector2 = Vector2::new(3.0,4.0);
        let comp : f32 = 5.0;
        let test: f32 = v1.magnitude();
        assert_eq!(test, comp);
    }
    #[test]
    fn test_vector_scale() {
        let v1     : Vector2 = Vector2::new(2.0,3.0);
        let scale      : f32 = 3.141; 
        let v_comp : Vector2 = Vector2::new(6.282, 9.423);
        let v_test : Vector2 = v1 * scale;
        assert_eq!(v_test,v_comp);
    }
    #[test]
    fn test_vector_dot_product() {
        let v1     : Vector2 = Vector2::new(2.0,3.0);
        let v2     : Vector2 = Vector2::new(1.0,8.0);
        let comp : f32 = 26.0;
        let test : f32 = v1.dot_product(v2);
        assert_eq!(test, comp);
    }
    #[test]
    fn test_vector_perpendicular() {
        let v1     : Vector2 = Vector2::new(2.0,3.0);
        let v_comp : Vector2 = Vector2::new(3.0,-2.0);
        let v_test : Vector2 = v1.perpendicular();
        assert_eq!(v_test, v_comp);
    }
    #[test]
    fn test_vector_normalise() {
        let v1     : Vector2 = Vector2::new(3.0,4.0);
        let v_comp : Vector2 = Vector2::new(0.6,0.8);
        let v_test : Vector2 = v1.normalise();
        assert_eq!(v_test, v_comp);
    }
    #[test]
    fn test_vector_rotate() {

        let pi : f32 = 3.1416;
        let v1 : Vector2 = Vector2::new(2.0, 2.0);
        let v_comp :  Vector2 = Vector2 ::new(-2.0000072, 1.9999927);
        let v_test = v1.rotate(pi/2.0);
        assert_eq!(v_test, v_comp);
    }
}