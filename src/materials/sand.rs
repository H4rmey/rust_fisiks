
use super::matter::{Matter, MatterState};

const MELTING_POINT : f32 = 1550f32;
const BOILING_POINT : f32 = 2230f32;
const COLOR : [f32 ; 4] = [
                            238.0 / 255.0, 
                            194.0 / 255.0, 
                            22.0 / 255.0, 
                            1.0
                        ];

pub struct Sand{
    temperature : f32,
}

impl Sand{
    pub fn new(temperature : f32) -> Self{
        Sand{
            temperature : temperature,
        }
    }

}

impl Matter for Sand{
    fn set_temperature(&mut self, temperature : f32){
        self.temperature = temperature;
    }

    fn get_state(&self) -> MatterState {
        if self.temperature >= BOILING_POINT {
            return MatterState::GAS;
        }else if self.temperature >= MELTING_POINT {
            return MatterState::LIQUID;
        }

        return MatterState::SOLID;
    }

    fn get_color(&self) -> [f32; 4]{
        return COLOR;
    }
}