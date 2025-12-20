struct RenderEntity {
    x: f64,
    y: f64,
    orientation: f64,
    dots: Vec<f64>, // local-space dots
}

impl RenderEntity
{
    pub fn new() -> self
    {
        self
        {
            x:0.0,
            y:0.0,
            orientation:0.0
            dots : Vec<f64>::new()
        }
    }
}