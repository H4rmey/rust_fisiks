
use core::f64;
use std::{convert::TryInto, fs::File};
use std::io::prelude::*;
use std::path::Path;

use rand::prelude::StdRng;
use rand::{Rng, SeedableRng};

use json::*;

pub struct Settings
{
    pub json: JsonValue,
}

impl Settings
{
    pub fn new(path: &str) -> Settings
    {
        let s = Settings
        {
            json:  Settings::get_settings(path),
        };

        s
    }

    fn get_settings(a_path : &str) -> JsonValue
    { 
        // Create a path to the desired file
        let path = Path::new(&a_path);
        let display = path.display();

        // Open the path in read-only mode, returns `io::Result<File>`
        let mut file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", display, why),
            Ok(file) => file,
        };

        // Read the file contents into a string, returns `io::Result<usize>`
        let mut settings = String::new();
        match file.read_to_string(&mut settings) {
            Err(why) => panic!("couldn't read {}: {}", display, why),
            Ok(_) => print!("{} contains:\n{}\n", display, settings),
        }
        
        parse(&settings[..]).unwrap()
    }

    pub fn get_random_value(&self, range_type: &str, attr: &str) -> f64
    {         
        let mut rng = rand::thread_rng();
        let min = self.json["ranges"][range_type][attr][0].as_f64().unwrap();
        let max = self.json["ranges"][range_type][attr][1].as_f64().unwrap();

        rng.gen_range(min..=max)
    }

    pub fn get_range_values(&self, range_type: &str, attr: &str) -> [f64; 2]
    {         
        let min = self.json["ranges"][range_type][attr][0].as_f64().unwrap();
        let max = self.json["ranges"][range_type][attr][1].as_f64().unwrap();

        [min, max]
    }
}

