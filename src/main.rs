extern crate piston_window;

mod grass;
mod pid_line;

use pid_line::PID;
use piston_window::*;
use cgmath::*;
use grass::*;

use std::{f64::consts::PI};


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

    // let mut grass: PidLine = PidLine::new(Vector2::new(320.0, 400.0), PI/4f64, 80f64, 2f64);
    
    let mut mouse_position: Vector2<f64> = Vector2::new(0f64,0f64);

    let mut left_click:     bool = false;
    let mut right_click:    bool = false;
    let     force:  f64 = 0.01f64;
        
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

    while let Some(e) = window.next() 
    {
        if let Some(_) = e.render_args()
        {
            window.draw_2d(&e, |c, g, _d| 
            {
                clear([0.0, 0.0, 0.0, 0.0], g);
                
                grass.draw(c, g);
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

                let l: f64 = grass.position.distance(mouse_position);
                let from_min: f64   = 0_f64;
                let from_max: f64   = grass.total_line_length as f64;
                let to_min: f64     = 0_f64;
                let to_max: f64     = 10_f64;
                grass.bendfactor = to_min + (l-from_min)/(from_max - from_min)*(to_max-to_min);
            }

            if right_click 
            {
                grass.position.distance(mouse_position);
                
                grass.pid_lines[0].pid.integral = -force;
            }
              
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


        if let Some(Button::Mouse(MouseButton::Right)) = e.release_args()
        {
            right_click = false;
        }

        if let Some(Button::Mouse(MouseButton::Right)) = e.press_args()
        {
            right_click = true;
        }
    }
}