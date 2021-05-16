extern crate piston_window;

mod grass;
mod pid_line;
mod wind_particle;

use piston_window::*;
use cgmath::*;

use std::{f64::consts::PI};

use grass::*;
use wind_particle::*;
use pid_line::PID;


fn main() 
{
    #![allow(non_snake_case)]
    let WIDTH   : f64 = 1280f64;
    let HEIGHT  : f64 = 720f64;

    let mut window: PistonWindow = WindowSettings::new("Hello Piston!", (WIDTH, HEIGHT))
            .exit_on_esc(true)
            .resizable(false)
            .build()
            .unwrap_or_else(|e| { panic!("Failed to build PistonWindow: {}", e) });
    
    let mut mouse_position: Vector2<f64> = Vector2::new(0f64,0f64);

    let mut left_click:     bool = false;
    let mut right_click:    bool = false;
        
    let pid_controller : PID = PID{
        error      : 0f64,
        error_old  : 0f64,
        derivative : 0f64,
        integral   : 0f64,
    
        kp  : 0.97f64,
        ki  : 0.4f64,
        kd  : 0.001f64,
    };

    let mut grass: Grass = Grass::new(
                            350, 
                            6, 
                            1.6f64,
                            Vector2::new(WIDTH/2f64, 600.0),
                            4f64,
                            pid_controller,
                            2f64
                        );
    
    grass.init();
    
    let mut wind: WindParticle = WindParticle::new(
                                            500_f64, 
                                            Vector2::new(0_f64, HEIGHT/2_f64+100_f64),
                                            -PI/40_f64,
                                            [0.0, 0.0, 0.8, 1.0],
                                            [WIDTH, HEIGHT]
                                        );
    

    while let Some(e) = window.next() 
    {
        if let Some(_) = e.render_args()
        {
            window.draw_2d(&e, |c, g, _d| 
            {
                clear([0.0, 0.0, 0.0, 0.0], g);
                
                grass.draw(c, g);  
                wind.draw(c, g);
            });
        }     

        if let Some(u) = e.update_args()
        {        
            if left_click
            {
                let y: f64 = mouse_position.y - grass.position.y;
                let x: f64 = mouse_position.x - grass.position.x;

                grass.pid_lines[0].angle            = y.atan2(x) + PI/2f64;
                grass.pid_lines[0].pid.integral     = 0.0;
                grass.pid_lines[0].pid.derivative   = 0.0;
                grass.pid_lines[0].pid.error        = 0.0;
            }

            wind.update(u);
            grass.update_wind(u, &wind);
              
            grass.update(u);
        }

        if let Some(args) = e.mouse_cursor_args()
        {
            mouse_position.x = (args[0].round()) as f64;
            mouse_position.y = (args[1].round()) as f64;
        }

        if let Some(Button::Mouse(MouseButton::Left)) = e.release_args()
        {
            left_click = false;
        }

        if let Some(Button::Mouse(MouseButton::Left)) = e.press_args()
        {
            left_click = true;
        }
    }
}