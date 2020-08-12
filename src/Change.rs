use crate::Image;
use crate::Utils::Util;

use std::convert::TryInto;

#[derive(Default)]
pub struct Change {
    image_queue: Vec<Image>,
    sln: i32,
    eln: i32,
    increments: i32,
    pub total_image_count: i32
}

impl Change {
    pub fn create_new(images: Vec<Image>, sln: i32, pln: i32, start: i32, end: i32) -> Change {
       let c: Change = Default::default();
        
        return c;
    }
    // Gets start image of change sequence
    pub fn get_start_list_num(self) -> Option<i32> {
        return Some(self.sln);
    }
    pub fn get_end_list_num(self) -> Option<i32> {
        return Some(self.eln);
    }
    pub fn get_total_images(self) -> Option<i32> {
        return Some(self.total_image_count);
    }
    pub fn set_start(mut c:Change, start: i32) {
        c.sln = start;
    }
    pub fn set_end(mut c: Change, end: i32) {
        c.eln = end;
    }

    pub fn update_metadata(self, key: String) {
        let tmp = self.image_queue.iter();
        let i = 0;
        let t: &Image;
        while i < self.sln {
             t = tmp.next().unwrap().to_owned();
        }
        let start = Util::find(t.get_xmp().unwrap(), key);
        let a: i32 = 0;
        while a < self.eln {
            let curr = tmp.nth(a.try_into().unwrap()).unwrap();
            let file = curr.get_xmp().unwrap();
            let new_val: i32 = start.parse::<i32>().unwrap() + (self.increments * (a - self.sln));
            let new_data = Util::replace(file, key, new_val.try_into().unwrap());
            
            if key == "Exposure" {
                let exposure = Util::find(t.get_xmp().unwrap(), "Exposure2012".to_string());
                let e = exposure.parse::<u64>().unwrap();
                curr.set_exposure(e);
            }

            else if key == "Temperature" {
                let white_balance = Util::find(t.get_xmp().unwrap(), "Temperature".to_string());
                let w = white_balance.parse::<u32>().unwrap();
                curr.set_white_balance(w);
            }

            Util::write_file_from_ref(curr, new_data);
        }
    }
}
