extern crate piston_window;

mod grass;
mod pid_line;

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
    let mut is_pressed: bool = false;
        
    let mut grass: Grass = Grass::new(
                            350, 
                            5, 
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
            if !is_pressed 
            {
                grass.update(u);
            }
            else
            {
                let y: f64 = mouse_position.y - grass.position.y;
                let x: f64 = mouse_position.x - grass.position.x;
                grass.update_angle(y.atan2(x) + PI/2f64);
                // grass.pid.log();
                // print!("{}, {}\n", mouse_position.x, mouse_position.y);
            }  
            
            // grass.position.x += 10f64 * u.dt;
        }

        if let Some(args) = e.mouse_cursor_args()
        {
            mouse_position.x = (args[0].round()) as f64;
            mouse_position.y = (args[1].round()) as f64;
        }

        if let Some(Button::Mouse(MouseButton::Left)) = e.release_args()
        {
            is_pressed = false;
        }

        if let Some(Button::Mouse(MouseButton::Left)) = e.press_args()
        {
            is_pressed = true;
        }
    }
}