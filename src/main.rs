extern crate piston_window;

mod grass;
mod pid_line;
mod wind_particle;
mod settings;

use piston_window::*;
use cgmath::*;

use std::{f64::consts::PI};

use grass::*;
use wind_particle::*;
use pid_line::PID;
use settings::*;

fn init(
        setting        : &mut Settings,  
        grass_field    : &mut Vec<Grass>,
        wind_field     : &mut Vec<WindParticle>
    )
{
    *setting = Settings::new("./settings.json");

    let width       : f64 = setting.json["window_width"].as_f64().unwrap();
    let height      : f64 = setting.json["window_height"].as_f64().unwrap();
    let grass_count : u64 = setting.json["grass_count"].as_u64().unwrap();

    *grass_field = Vec::new();
    for i in 0..grass_count
    {
        /*create a pid controller */
        let pid_controller : PID = PID
        {
            error      : 0f64,
            error_old  : 0f64,
            derivative : 0f64,
            integral   : 0f64,
            
            kp  : setting.get_random_value("grass", "kp"),
            ki  : setting.get_random_value("grass", "ki"),
            kd  : setting.get_random_value("grass", "kd"),
        }; 

        let grass: Grass = Grass::new(
                setting.get_random_value("grass", "total_line_length") as u64, 
                setting.get_random_value("grass", "line_amount") as usize, 
                setting.get_random_value("grass", "ratio"),
                Vector2::new(i as f64 * (width/grass_count as f64), 600.0),
                setting.get_random_value("grass", "radius"),
                pid_controller
            );

        grass_field.push(grass);
    }

    /*init the grass parts */
    for i in 0..grass_count as usize
    {
        grass_field[i].init();
    }
    
    *wind_field = Vec::new();
    for _ in 0..setting.json["wind_partical_count"].as_u64().unwrap()
    {
        let wind: WindParticle = WindParticle::new(
                setting.get_random_value("wind", "speed"), 
                Vector2::new(0_f64, setting.get_random_value("wind", "height")),
                setting.get_random_value("wind", "angle") * PI/180_f64,
                [0.0, 0.0, 0.8, 1.0],
                [width, height]
            );
        wind_field.push(wind);
    }
}

fn main() 
{
    let mut setting: Settings = Settings::new("./settings.json");
    let mut is_pressed: bool = false;
    let mut press_flag: bool = true;

    let mut window: PistonWindow = WindowSettings::new("Hello Piston!", (1280, 720))
            .exit_on_esc(true)
            .resizable(false)
            .build()
            .unwrap_or_else(|e| { panic!("Failed to build PistonWindow: {}", e) });
     


    /*create a grass field */
    let mut grass_field: Vec<Grass> = Vec::new();    
    let mut wind_field: Vec<WindParticle> = Vec::new();
    
    init(&mut setting, &mut grass_field, &mut wind_field);

    while let Some(e) = window.next() 
    {
        if let Some(_) = e.render_args()
        {
            window.draw_2d(&e, |c, g, _d| 
            {
                clear([0.0, 0.0, 0.0, 0.0], g);
                
                for i in 0..grass_field.len()
                {
                    grass_field[i].draw(c, g);
                } 

                for i in 0..wind_field.len()
                {
                    wind_field[i].draw(c, g);
                }

                rectangle([0.59, 0.29, 0.0, 1.0], [0.0, 590.0, 1280.0, 720.0], c.transform, g);
            });
        }     

        if let Some(u) = e.update_args()
        {   
            for i in 0..wind_field.len()
            {
                wind_field[i].update(u);
            }

            for i in 0..grass_field.len()
            {
                for p in 0..wind_field.len()
                {
                    grass_field[i].update_wind(u, &wind_field[p], &setting);
                }              
                grass_field[i].update(u);
            } 

            if is_pressed && press_flag
            {
                press_flag = false;
                init(&mut setting, &mut grass_field, &mut wind_field);
            }

            if !is_pressed
            {
                press_flag = true;

            }
        }

        if let Some(Button::Mouse(MouseButton::Left)) = e.release_args() {
            is_pressed = false;
        }

        if let Some(Button::Mouse(MouseButton::Left)) = e.press_args() {
            is_pressed = true;
        }
    }
}