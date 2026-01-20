pub struct Camera {
    pub x: f64,
    pub y: f64,
    pub ppm : f64 // pixels per meter
}

impl Camera{
    pub fn new() -> Self
    {
        Self
        {
            x:0.0,
            y:0.0,
            ppm: 1.0,
        }
    }
}