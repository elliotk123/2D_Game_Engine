use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;
use std::ops::SubAssign;

#[derive(Debug, Copy, Clone)]
pub struct Vector2
{
    pub x  : f64,
    pub y  : f64
}
impl Vector2
{
    pub fn new(x_val : f64, y_val : f64) -> Vector2
    {
        Vector2
        {
            x : x_val,
            y : y_val
        }
    }
    pub fn magnitude(self) -> f64
    {
        return (self.x.powf(2.0) + self.y.powf(2.0)).sqrt();
    }
    pub fn scale(self, value : f64) -> Vector2
    {
        Vector2
        {
            x : self.x * value,
            y : self.y * value
        }
    }
    pub fn dot_product(self, rhs : Vector2) -> f64
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
        let length : f64 = self.magnitude();
        return  Vector2
                {
                    x : self.x / length,
                    y : self.y / length
                }
    }
    pub fn rotate(self, theta: f64) -> Vector2
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
impl Mul<f64> for Vector2
{
    type Output = Vector2;
    fn mul(self,_rhs : f64) -> Vector2
    {
        Vector2{x: self.x,y: self.y}.scale(_rhs)
    }
}
impl Mul<i32> for Vector2
{
    type Output = Vector2;
    fn mul(self,_rhs : i32) -> Vector2
    {
        Vector2{x: self.x,y: self.y}.scale(_rhs as f64)
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