
use piston_window::*;
use cgmath::*;

use crate::pid_line::PidLine;
use std::{f64::consts::PI};


pub struct Grass
{
    pub pid_lines           : Vec<PidLine>,
    part_lengths            : Vec<f64>,

    pub total_line_length   : u64, 
    pub line_amount         : usize, 
    pub ratio               : f64,
    pub position            : Vector2<f64>,
    pub radius              : f64
}

impl Grass
{
    pub fn new(
            total_line_length   : u64, 
            line_amount         : usize, 
            ratio               : f64,
            position            : Vector2<f64>,
            radius              : f64
        )-> Grass
    {
        Grass
        {
            pid_lines           : Vec::new(),
            part_lengths        : vec![0f64; line_amount],

            total_line_length   : total_line_length,
            line_amount         : line_amount,
            ratio               : ratio,
            position            : position,
            radius              : radius
        }
    }

    #[allow(dead_code)]
    fn calc_lengths(
            total_line_length: u64, 
            line_amount: usize, 
            ratio : f64
        ) -> Vec<f64>
    {
        let mut l: Vec<f64> = Vec::new();

        let mut sum: f64 = 0f64;
        for i in (0..line_amount).rev()
        {
            let value: f64 = ratio.powf(i as f64 + 1f64);
            l.push(value);

            sum += value;
        }

        let r: f64 = total_line_length as f64/sum;
        for i in 0..line_amount
        {
            l[i] *= r;
        }

        l
    }

    pub fn draw(&mut self, c : Context, g : &mut G2d)
    {
        for i in 0..self.line_amount
        {
            self.pid_lines[i].draw(c, g);
        }
    }

    pub fn init(&mut self)
    {
        // self.part_lengths = vec![128f64,64f64,32f64,16f64, 8f64];
        self.part_lengths = Grass::calc_lengths(
                                        self.total_line_length,
                                        self.line_amount,
                                        self.ratio
                                    );
        for i in 0..self.line_amount
        {
            self.pid_lines.push(PidLine::new(
                                        Vector2::new(320.0, 400.0), 
                                        PI/4f64, 
                                        self.part_lengths[i], 
                                        self.radius 
                                    )
                                );
        }
    }

    pub fn update(&mut self, u: UpdateArgs)
    {   
        for i in 0..self.line_amount
        {
            self.pid_lines[i].length = self.part_lengths[i];
            self.pid_lines[i].radius = self.radius;

            if i == 0
            {
                self.pid_lines[i].position = self.position;
            }
            else
            {
                self.pid_lines[i].position = self.pid_lines[i-1].end_point;
            }

            self.pid_lines[i].update(u);
        }
    } 
    
    pub fn update_angle(&mut self, angle : f64)
    {
        for i in 0..self.line_amount
        {
            self.pid_lines[i].angle = angle;
        }
    }
}