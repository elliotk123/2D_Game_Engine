pub struct RenderEntity {
    pub x: f64,
    pub y: f64,
    pub layer_height:f64,
    pub orientation: f64,
    pub dots: Vec<f64>, // local-space dots
    pub sprite_key : String,
    pub colour_id : u16,
}


impl RenderEntity
{
    pub fn new() -> Self
    {
        Self::default()
    }
}

impl Default for RenderEntity{
    fn default() -> Self {
        Self {
            x:0.0,
            y:0.0,
            layer_height:0.0,
            orientation:0.0,
            dots : Vec::<f64>::new(),
            sprite_key: String::new(),
            colour_id : 0,
        }
    }
}