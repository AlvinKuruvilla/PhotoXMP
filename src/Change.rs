use crate::Image;

pub struct Change {
    image_queue: Vec<Image>,
    sln: i32,
    pln: i32,
    pub total_image_count: i32
}

impl Change {
    pub fn create_new(images: Vec<Image>, sln: i32, pln: i32, start: i32, end: i32) -> Change {
        let change = Change {
            image_queue: images,
            sln: sln,
            pln: pln,
            total_image_count: (end-start)+1
        };
        return change;
    }
    // Gets start image of change sequence
    pub fn get_start_list_num(self) -> Option<i32> {
        return Some(self.sln);
    }
    pub fn get_end_list_num(self) -> Option<i32> {
        return Some(self.pln);
    }
    pub fn get_total_images(self) -> Option<i32> {
        return Some(self.total_image_count);
    }
    pub fn set_start(mut c:Change, start: i32) {
        c.sln = start;
    }
    pub fn set_end(mut c:Change, end: i32) {
        c.pln = end;
    }
    pub fn update_metadata(key:String) {

    }
}
