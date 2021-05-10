
use piston_window::*;
use cgmath::*;

use std::f64::consts::PI;

pub struct PID
{
    pub error       : f64,
    pub error_old   : f64,
    pub derivative  : f64,
    pub integral    : f64,

    pub kp  : f64,
    pub kd  : f64,
    pub ki  : f64,
}

pub struct Grass
{
    pub grass   : Line,
    pub origin  : Vector2<f64>,
    pub angle   : f64,
    pub length  : f64,
    pub radius  : f64, 
    pub pid     : PID,
}

fn new_vec2_with_angle(len: f64, angle: f64 /*in radians*/) -> Vector2<f64>
{
    Vector2::new(angle.cos() * len, angle.sin() * len)    
}

impl PID
{
    #[warn(dead_code)]
    pub fn log(&self)
    {
        println!("error       : {:?}", &self.error       );
        println!("error_old   : {:?}", &self.error_old   );
        println!("derivative  : {:?}", &self.derivative  );
        println!("integral    : {:?}", &self.integral    );
    }
}

impl Grass
{
    pub fn new(
            origin  : Vector2<f64>,
            angle   : f64,
            length  : f64,
            radius  : f64
        ) -> Grass
    {
        let l: Line = Line {
            color   : [0.0, 0.8, 0.0, 1.0], 
            radius  : radius,
            shape   : line::Shape::Square,
        };

        let pid: PID = PID
        {
            error      : 0f64,
            error_old  : 0f64,
            derivative : 0f64,
            integral   : 0f64,
        
            kp  : 0.97f64,
            ki  : 0.4f64,
            kd  : 0.001f64,
        };

        let grass: Grass = Grass
        {
            grass   : l,
            origin  : origin,
            angle   : angle,
            length  : length,
            radius  : radius,
            pid     : pid,
        };

        grass
    }

    pub fn update(&mut self, u: UpdateArgs)
    {        
        self.pid.error      = self.angle;
        self.pid.error_old  = self.angle;
        
        self.pid.error      =   self.pid.kp * self.pid.error    + 
                                self.pid.ki * self.pid.integral + 
                                self.pid.kd * self.pid.derivative;

        self.pid.derivative = self.pid.error - self.pid.error_old;
        self.pid.integral   += -self.pid.error * u.dt;

        self.angle = self.pid.error;
    }

    pub fn draw(&self, c : Context, g : &mut G2d)
    {        
        let start: Vector2<f64> = self.origin;  

        let end: Vector2<f64> = new_vec2_with_angle(self.length, -self.angle + PI/2f64);

        self.grass.draw(
                [start.x, start.y, start.x + end.x, start.y - end.y], 
                &Default::default(), 
                c.transform, 
                g
            );
    }
}