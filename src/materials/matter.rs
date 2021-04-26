
pub enum MatterState{
    SOLID,
    LIQUID,
    GAS
}

pub trait Matter{
    fn set_temperature(&mut self, temperature : f32);
    fn get_state(&self, ) -> MatterState;


    fn get_color(&self) -> [f32; 4];
}