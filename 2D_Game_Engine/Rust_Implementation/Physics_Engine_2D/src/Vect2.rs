use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;

#[derive(Debug, Copy, Clone)]
pub struct Vector2
{
    x  : f32,
    y  : f32
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
    pub fn dotProduct(self, other : Vector2) -> f32
    {
        return (self.x * other.x) + (self.y * other.y);
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
    fn sub(self, _other : Vector2) -> Vector2
    {
        Vector2
        {
            x: self.x - _other.x,
            y: self.y - _other.y
        }
    }
}
impl Add for Vector2
{
    type Output = Vector2;
    fn add(self, _other: Vector2) -> Vector2
    {
        Vector2
        {
            x: self.x + _other.x,
            y: self.y + _other.y
        }
    }
}
impl Mul for Vector2
{
    type Output = Vector2;
    fn mul(self, _other: Vector2) -> Vector2
    {
        Vector2
        {
            x: self.x * _other.x,
            y: self.y * _other.y
        }
    }
}
impl Div for Vector2
{
    type Output = Vector2;
    fn div(self, _other: Vector2) -> Vector2
    {
        Vector2
        {
            x: self.x / _other.x,
            y: self.y / _other.y
        }
    }
}