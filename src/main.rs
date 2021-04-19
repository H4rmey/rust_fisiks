extern crate piston_window;

use piston_window::*;

const WINDOW_WIDTH : u32 = 640;
const WINDOW_HEIGTH : u32 = 480;

fn main() 
{
    let mut window: PistonWindow =
        WindowSettings::new("Window", [WINDOW_WIDTH, WINDOW_HEIGTH])
        .exit_on_esc(true).build().unwrap();
    while let Some(event) = window.next() 
    {
        window.draw_2d(&event, |context, graphics, _device| 
        {
            clear([1.0; 4], graphics);
            for x in 0..WINDOW_WIDTH/2
            {
                for y in 0..WINDOW_HEIGTH/2
                {
                    rectangle([1.0, 0.0, 0.0, 1.0], // red
                            [x as f64 * 2.0, y as f64 * 2.0, 1.0, 1.0],
                            context.transform,
                            graphics);
                }
            }
            
            clear([1.0; 4], graphics);
            for x in 1..WINDOW_WIDTH/2-1
            {
                for y in 1..WINDOW_HEIGTH/2-1
                {
                    rectangle([1.0, 0.0, 0.0, 1.0], // red
                            [x as f64 * 2.0, y as f64 * 2.0, 1.0, 1.0],
                            context.transform,
                            graphics);
                }
            }
        });
    }
}