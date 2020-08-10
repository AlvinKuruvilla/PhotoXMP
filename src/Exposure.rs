use std::convert::TryFrom;
use crate::Image;
use crate::Change::Change;

pub struct ExposeChange {
    image_queue: Vec<Image>,
    sln: i32,
    pln: i32,
    expo: i64, 
}

impl ExposeChange {
    pub fn create_new(_c: Change, imgs: Vec<Image>, sln: i32, pln: i32, exposure_change: i64) -> ExposeChange {
        let expc = ExposeChange {
            //todo: combine the two image vectors together 
            image_queue: imgs,
            sln: sln,
            pln: pln,
            expo: exposure_change
        };
        return expc
    }
    pub fn get_exposure(self) -> Option<i64> {
        return Some(self.expo);
    }
    pub fn set_exposure(mut e: ExposeChange, new_exposure: i64) {
        e.expo = new_exposure;
    }
    pub fn set_increment( self, c: Change) {
        let tmp = i64::try_from(c.total_image_count).ok().unwrap();
        let increments = self.expo/tmp;
    }
}