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
    const WIDTH   : f64 = 1280f64;
    const HEIGHT  : f64 = 720f64;

    const GRASS_COUNT : usize = 250;

    let mut window: PistonWindow = WindowSettings::new("Hello Piston!", (WIDTH, HEIGHT))
            .exit_on_esc(true)
            .resizable(false)
            .build()
            .unwrap_or_else(|e| { panic!("Failed to build PistonWindow: {}", e) });
    
    let mut mouse_position: Vector2<f64> = Vector2::new(0f64,0f64);

    let mut left_click:     bool = false;
        
    let pid_controller : PID = PID{
        error      : 0f64,
        error_old  : 0f64,
        derivative : 0f64,
        integral   : 0f64,
    
        kp  : 0.97f64,
        ki  : 0.4f64,
        kd  : 0.001f64,
    };

    let mut grass_field: Vec<Grass> = Vec::new();
    for i in 0..GRASS_COUNT
    {
        let grass: Grass = Grass::new(
                                80, 
                                5, 
                                1.1f64,
                                Vector2::new(i as f64 * (WIDTH/GRASS_COUNT as f64), 600.0),
                                1.5f64,
                                pid_controller,
                                3f64
                            );

        grass_field.push(grass);
    }

    for i in 0..GRASS_COUNT
    {
        grass_field[i].init();
    }
    
    let mut wind: WindParticle = WindParticle::new(
                                            1000_f64, 
                                            Vector2::new(0_f64, 600.0),
                                            -PI/400_f64,
                                            [0.0, 0.0, 0.8, 1.0],
                                            [WIDTH, HEIGHT]
                                        );
    let mut wind2: WindParticle = WindParticle::new(
                                            600_f64, 
                                            Vector2::new(0_f64, 500.0),
                                            PI/400_f64,
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
                
                for i in 0..GRASS_COUNT
                {
                    grass_field[i].draw(c, g);
                } 
                wind.draw(c, g);
                wind2.draw(c, g);

                rectangle([0.59, 0.29, 0.0, 1.0], [0.0, 600.0, WIDTH, HEIGHT], c.transform, g);
            });
        }     

        if let Some(u) = e.update_args()
        {        
            if left_click
            {
                // let y: f64 = mouse_position.y - grass.position.y;
                // let x: f64 = mouse_position.x - grass.position.x;

                // grass.pid_lines[0].angle            = y.atan2(x) + PI/2f64;
                // grass.pid_lines[0].pid.integral     = 0.0;
                // grass.pid_lines[0].pid.derivative   = 0.0;
                // grass.pid_lines[0].pid.error        = 0.0;
            }

            wind.update(u);
            wind2.update(u);

            for i in 0..GRASS_COUNT
            {
                grass_field[i].update_wind(u, &wind);
                grass_field[i].update_wind(u, &wind2);
                grass_field[i].update(u);
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
    }
}