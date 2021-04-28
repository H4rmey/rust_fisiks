extern crate piston_window;

use piston_window::*;

fn main() 
{
    let mut window: PistonWindow = WindowSettings::new("Hello Piston!", (640, 480))
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .unwrap_or_else(|e| { panic!("Failed to build PistonWindow: {}", e) });


    let mut x: f64 = 0f64;
    while let Some(e) = window.next() 
    {
        if let Some(u) = e.render_args()
        {
            window.draw_2d(&e, |c, g, _d| 
            {
                clear([0.0, 0.0, 0.0, 0.0], g);
                                
                let l: Line = Line {
                    color: [0.0, 0.5, 0.0, 1.0], 
                    radius: 5f64,
                    shape: line::Shape::Square,
                };
                
                x += 1f64;
                l.draw([320.0, 480.0, x, 240.0], &Default::default(), c.transform, g);

                // line(
                //    [0.0, 0.5, 0.0, 1.0], 
                //    5f64,
                //    [320.0, 480.0, 320.0, 240.0],
                //    c.transform,
                //    g                   
                // );
            });
        }

        if let Some(_) = e.update_args()
        {
            
        }
    }
}