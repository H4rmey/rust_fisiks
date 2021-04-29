extern crate piston_window;

use std::f64::consts::PI;

use piston_window::*;
use cgmath::*;

fn new_vec2_with_angle(len: f64, angle: f64 /*in radians*/) -> Vector2<f64>
{
    let x: f64 = angle.cos() * len;
    let y: f64 = angle.sin() * len;
    let vec: Vector2<f64> = Vector2::new(x, y);

    vec
}
 
fn main() 
{
    let mut window: PistonWindow = WindowSettings::new("Hello Piston!", (640, 480))
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .unwrap_or_else(|e| { panic!("Failed to build PistonWindow: {}", e) });

    let mut angle:      f64     = PI/4f64; 
    let mut angle_old:  f64     = 0f64;

    let kp:             f64     = 0.97f64;
    let ki:             f64     = 0.4f64;
    let kd:             f64     = 0.001f64;

    let mut error:      f64     = 0f64;
    let mut integral:   f64     = 0f64;
    let mut derivative: f64     = 0f64;

    let mut mouse_position: Vector2<f64> = Vector2::new(0f64,0f64);
    let mut is_pressed: bool = false;
        
    while let Some(e) = window.next() 
    {
        if let Some(_) = e.render_args()
        {
            window.draw_2d(&e, |c, g, _d| 
            {
                clear([0.0, 0.0, 0.0, 0.0], g);

                let start: Vector2<f64> = Vector2::new(320.0, 400.0);  

                let end: Vector2<f64> = new_vec2_with_angle(50f64, -angle + PI/2f64);

                let l: Line = Line {
                    color: [0.0, 0.8, 0.0, 1.0], 
                    radius: 1f64,
                    shape: line::Shape::Square,
                };
                
                l.draw([start.x, start.y, start.x + end.x, start.y - end.y], &Default::default(), c.transform, g);
            });
        }     

        if let Some(u) = e.update_args()
        {
            if !is_pressed 
            {
                angle_old   = angle;
                error       = angle;
                
                angle       = kp*error + ki*integral + kd*derivative;
    
                derivative  = angle - angle_old;
                integral    += -error * u.dt; 
            }
            else
            {
                angle = (mouse_position.y - 400f64 ).atan2(mouse_position.x - 320f64)+ PI/2f64;
                print!("{}, {}\n", mouse_position.x, mouse_position.y);
            }            
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