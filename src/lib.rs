extern crate log;

use crate::Utils::Util;

use log::info;

mod Exposure;
mod Change;
mod Utils;
mod Temperature;

pub use crate::Exposure::ExposeChange;
pub use crate::Temperature::TempChange;

use std::fs::File;
use std::any::type_name;
use std::path::Path;

use log::{warn};

pub struct PhotoXMP {
    util: Util,
    image_sequence: Vec<Image>,
    exposure_sequence: Vec<ExposeChange>,
    temperature_sequence: Vec<TempChange>,
    exposure_increment: i64,
    start: i32,
    end: i32
}

impl PhotoXMP {
    pub fn get_image_data(i: Image) {
        // Gets data from xmp file
       let xmp_path =  Image::get_xmp(i).unwrap();
       let _f = File::create(xmp_path);
    }
    fn type_of<T>(_: T) -> &'static str {
        type_name::<T>()
    }
    pub fn get_previous_image(current: Image, p: PhotoXMP)  -> Image {
        let v =  p.image_sequence.into_iter().find(|x| x == &current).expect("Nothing found");
        return v;
    }
    pub fn get_next_image(folder: String, curr_image: Image) {
        let image_name = Image::get_image_name(curr_image);
        
        let pre_num = String::new();
        let extension = String::new();
        let string_num = String::new();
        let mut count = 0;
        loop {
            count += 1;
            let comp = image_name.chars().nth(count);
            if  comp.unwrap().to_string()  == ".".to_owned() {
                    let mut j = count;
                    while j < image_name.len(){
                        let tmp = image_name.chars().nth(j).unwrap().to_string();
                        let s_slice: &str = &tmp[..];
                        extension.push_str(s_slice);
                    }
                break;
            }
            else if image_name.chars().nth(count).unwrap().is_digit(10) {
                let t = image_name.chars().nth(count).unwrap().to_string();
                let s: &str = &t[..];
                string_num.push_str(s);
            }
            else {
                let a = image_name.chars().nth(count).unwrap().to_string();
                let b: &str = &a[..];
                pre_num.push_str(b);
            }
        }
    }
    //todo: use failure here maybe
    pub fn retrieve_data(folder: String, start_name: String) -> bool {
        let img_arg = folder.clone()+ "\\" + &start_name; 

        let img = File::create(folder.clone()+ "\\" + &start_name);
        let xmp = File::create(folder.clone()+ "\\" + &Util::get_XMP_name(&start_name));
        let img_file_path = Path::new(&img_arg);
        let xmp_file_path = Path::new (&(folder+ "\\" + &Util::get_XMP_name(&start_name)));

        if !img_file_path.exists() {
            warn!("Image {:?} does not exist", img);
            return false;
        }
        //todo: REMOVE
        else {
            return true;
        }
      
         while img_file_path.exists() && xmp_file_path.exists() {
              return true;
          }
        
    }
}
#[derive(Debug, Eq, PartialEq)]
pub struct Image {
    xmp_path: String,
    image_path: String,
    iso: i32,
    list: u32,
    white_balance: u32,
    ratio: u64,
    aperture: u64,
    shutter: u64,
    exposure: u64,
}
//todo: add filepath attribute
impl Image {
    pub fn create_default(
        xmp_path: String,
        image_path: String,
        iso: i32,
        list: u32,
        ratio: u64,
        aperture: u64,
        shutter: u64,
        exposure: u64,
        white_balance: u32,
    ) -> Image {
        return Image {
            aperture,
            iso,
            list,
            white_balance,
            ratio,
            shutter,
            xmp_path,
            exposure,
            image_path: String::from(&Util::get_XMP_name(&xmp_path)),
        };
    }
    pub fn create_new_image(name: String, folder_name: String) -> Image {
        return Image {
            aperture: 0,
            iso: 0,
            exposure: 0,
            ratio: 0,
            shutter: 0,
            white_balance: 0,
            list: 0,
            image_path: folder_name.clone() + "\\" + &name,
            xmp_path: folder_name + "\\" + &Util::get_XMP_name(&name)
        }
    }
    //return a file object from the xmp path
    pub fn file_from_XMP(i: Image) -> std::io::Result<()> {
        let _f = File::create(i.xmp_path)?;
        Ok(())
    }
    pub fn file_from_img_path(i: Image) -> std::io::Result<()> {
        let _f = File::create(i.image_path)?;
        Ok(())
    }
    pub fn get_xmp(self) -> Option<String> {
        return Some(self.xmp_path);
    }
    pub fn get_iso(self) -> Option<i32> {
        return Some(self.iso);
    }
    pub fn get_list(self) -> Option<u32> {
        return Some(self.list);
    }
    pub fn get_white_balance(self) -> Option<u32> {
        return Some(self.white_balance);
    }
    pub fn get_exposure(self) -> Option<u64> {
        return Some(self.exposure);
    }
    pub fn get_shutter_speed(self) -> Option<u64> {
        return Some(self.shutter);
    }
    pub fn get_ratio(self) -> Option<u64> {
        return Some(self.ratio);
    }
    pub fn get_image_name(self) -> String{
        let img_path = self.image_path;
        // let path = Path::new(&img_path);
        let filename = Path::new(&img_path).file_name().unwrap().to_str().unwrap().to_owned();
        
        return filename;
    } 
    pub fn print_image_info(self) {
        info!("Image file path: {}", self.image_path);
        info!("XMP Path: {}", self.xmp_path);
        info!("White Balance: {}", self.white_balance);
        info!("Exposure: {}", self.exposure);
        info!("Aperture: {}", self.aperture);
        info!("ISO: {}", self.iso);
        info!("Shutter speed: {}", self.shutter);
    }
    
}
