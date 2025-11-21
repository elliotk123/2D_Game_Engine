use std::vec;
use crate::vector2::Vector2;

#[derive(Debug, Clone)]
pub struct Shape
{
    vertices    : Vec<Vector2>
}
impl Shape
{
    pub fn new(vertices:Vec<Vector2>)->Shape
    {
        Shape
        {
            vertices
        }
    }
}