#[path = "./tool.rs"]
mod tool;

use piston_window::*;
use cgmath::*;

use tool::*;

pub struct WindParticle
{
    ellipse     : Ellipse,   
    size        : f64,

    pub speed       : f64,
    pub position    : Vector2<f64>,
    pub direction   : f64,

    dir_vec         : Vector2<f64>,

    window_width    : f64,
    window_height   : f64,

    origin      : Vector2<f64>,
}

impl WindParticle
{
    pub fn new(
                speed       : f64,
                origin      : Vector2<f64>,
                direction   : f64,
                color       : [f32; 4],
                window_size : [f64; 2]
            ) -> WindParticle
    {
        let e = Ellipse::new(color);

        WindParticle
        {
            ellipse     : e,
            size        : 10_f64,

            speed       : speed,
            position    : origin,
            direction   : direction,
            
            dir_vec     : Tool::new_vec2_with_angle(speed,direction),

            window_width    : window_size[0],
            window_height   : window_size[1],
            origin          : origin,
        }
    }

    pub fn update(&mut self, u: UpdateArgs)
    {
        self.position.x += self.dir_vec.x * u.dt;
        self.position.y += self.dir_vec.y * u.dt;

        if self.position.x < 0_f64 || self.position.x > self.window_width
        {
            self.position = self.origin;
        }

        if self.position.y < 0_f64 || self.position.y > self.window_height
        {
            self.position = self.origin;
        }
    }

    pub fn draw(&self, c : Context, g : &mut G2d)
    {
        self.ellipse.draw(
            [self.position.x, self.position.y, self.size, self.size], 
            &Default::default(), 
            c.transform, 
            g
        );
    }
}