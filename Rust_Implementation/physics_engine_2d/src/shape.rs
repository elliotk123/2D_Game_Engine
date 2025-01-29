use crate::vector2::Vector2;

#[derive(Debug, Clone, PartialEq)]
pub struct Shape
{
    pub vertices    : Vec<Vector2>
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