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
    let mut left_click: bool = false;

    let mut right_click: bool = false;
    let mut right_click_flag : bool = true;
    let mut timer: f64 = 0.0;
    let mut pid_clone: PID = PID::empty();
        
    let mut grass: Grass = Grass::new(
                            350, 
                            6, 
                            2f64,
                            Vector2::new(WIDTH/2f64, 600.0),
                            4f64
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
            // grass.update(u);            
            grass.update(u, true);
        
            if left_click
            {
                let y: f64 = mouse_position.y - grass.position.y;
                let x: f64 = mouse_position.x - grass.position.x;
                grass.pid_lines[0].angle = y.atan2(x) + PI/2f64;
                grass.pid_lines[0].pid.integral = 0.0;
            }

            let force : f64 = 0.1;

            if right_click && right_click_flag
            {
                right_click_flag = false;

                grass.pid_lines[0].angle = force*PI/180f64;    

                grass.pid_lines[0].pid.kd *= -1f64;
                grass.pid_lines[0].pid.ki *= -10f64;
                grass.pid_lines[0].pid.kp *= -1f64;
            }

            if !right_click_flag
            {
                timer += u.dt;
            }
            if timer > force
            {
                right_click_flag = true;
                timer = 0f64;
                grass.pid_lines[0].pid.kd = grass.pid_lines[0].pid.kd.abs();
                grass.pid_lines[0].pid.ki = grass.pid_lines[0].pid.ki.abs()/10f64;
                grass.pid_lines[0].pid.kp = grass.pid_lines[0].pid.kp.abs();
            }
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