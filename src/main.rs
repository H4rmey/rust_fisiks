extern crate piston_window;

pub mod materials;

use materials::sand::{Sand};
use materials::{matter::Matter};

use piston_window::*;

const X_LIMIT : usize = 640;
const Y_LIMIT : usize = 480;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Hello Piston!", (640, 480))
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .unwrap_or_else(|e| { panic!("Failed to build PistonWindow: {}", e) });


    let mut board : Vec<Option<Box<dyn Matter>>> = vec![];

    for _ in 0..(X_LIMIT * Y_LIMIT){
        board.push(None);
    }

    let mut mouse_position = [0usize; 2];

    let mut is_pressed = false;


    while let Some(e) = window.next() {

        if let Some(args) = e.mouse_cursor_args(){
            mouse_position[0] = (args[0].round()) as usize;
            mouse_position[1] = (args[1].round()) as usize;
        }

        if let Some(Button::Mouse(MouseButton::Left)) = e.release_args(){
            is_pressed = false;
        }

        if let Some(Button::Mouse(MouseButton::Left)) = e.press_args(){
            is_pressed = true;
        }

        if is_pressed {
            let boxed_matter : Box<dyn Matter> = Box::new(Sand::new(25.0));

            board[mouse_position[0] + (mouse_position[1] * X_LIMIT)] = Some(boxed_matter);
        }

        if let Some(_args) = e.render_args(){
            window.draw_2d(&e, |c, g, _d| {
                clear([0.0, 0.0, 0.0, 0.0], g);

                for n in 0..(X_LIMIT * Y_LIMIT) {
                    if let Some(matter) =  &board[n]{
                        let x = n % X_LIMIT;
                        let y = n / X_LIMIT;
                        rectangle(matter.get_color(), [x as f64, y as f64, 1f64, 1f64], c.transform, g);
                    }
                  
                }
            });
        }

        if let Some(_) = e.update_args(){
            let mut coord_stack : Vec<(usize, usize)> = vec![];

           for n in 0..(X_LIMIT * (Y_LIMIT - 1)) {
                if let Some(_) = &board[n] {
                    let x = n % X_LIMIT;
                    let y = n / X_LIMIT;
                    &coord_stack.push((x,y));
                }             
           }

           for n in coord_stack {
                let index = n.0 + (n.1 * X_LIMIT);
                let below = index + X_LIMIT;

                if *(&board[below].is_none()){
                    board[index] = None;
                    board[below] = Some(Box::new(Sand::new(25f32)));
                }else if *(&board[below - 1].is_none()) && ((below - 1) % X_LIMIT) > 0{
                    board[index] = None;
                    board[below - 1] = Some(Box::new(Sand::new(25f32)));
                }else if *(&board[below + 1].is_none()) && ((below + 1) % X_LIMIT) < (X_LIMIT - 1){
                    board[index] = None;
                    board[below + 1] = Some(Box::new(Sand::new(25f32)));
                }
           }
        }
    }
}