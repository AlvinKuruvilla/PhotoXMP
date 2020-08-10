use crate::Change::Change;
use crate::Image;

pub struct TempChange {
    image_queue: Vec<Image>,
    sln: i32,
    pln: i32,
    temperature: i32, 
}

impl TempChange {
    pub fn create_new(_c: Change, imgs: Vec<Image>, sln: i32, pln: i32, temperature_change: i32) -> TempChange {
        let tc = TempChange {
            //todo: combine the two image vectors together 
            image_queue: imgs,
            sln: sln,
            pln: pln,
            temperature: temperature_change
        };
        return tc;
    }
    pub fn get_temperature(self) -> Option<i32> {
        return Some(self.temperature);
    }
    pub fn set_temperature(mut self, new_temperature: i32) {
        self.temperature = new_temperature;
    }
}
