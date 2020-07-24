extern crate log;
use std::fs;
use std::fs::metadata;
use std::fs::File;
use std::convert::TryFrom;
use std::any::type_name;
use std::path::Path;

use log::{warn, info};

pub struct PhotoXMP {
    util: Util,
    image_sequence: Vec<Image>,
    exposure_sequence: Vec<ExposeChange>,
    temperature_sequence: Vec<TempChange>,
    exposure_increment: i64,
    start: i32,
    end: i32
}

#[derive(Debug)]
pub struct Util;

#[derive(Clone, Eq, PartialEq)]
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
pub struct ExposeChange {
    image_queue: Vec<Image>,
    sln: i32,
    pln: i32,
    expo: i64, 
}
pub struct TempChange {
    image_queue: Vec<Image>,
    sln: i32,
    pln: i32,
    temperature: i32, 
}
pub struct Change {
    image_queue: Vec<Image>,
    sln: i32,
    pln: i32,
    total_image_count: i32
}
pub trait IncrementChange {
    fn get_increment(&self, c:Change) -> u32;
}

impl PhotoXMP {
    pub fn get_image_data(i: Image) {
        // Gets data from xmp file
       let xmp_path =  Image::get_xmp(i);
       let _f = File::create(xmp_path);

    }
    pub fn type_of<T>(_: T) -> &'static str {
        type_name::<T>()
    }
    pub fn get_previous_image(current: Image, p: PhotoXMP)  -> Image {
        let v =  p.image_sequence.into_iter().find(|x| x == &current).expect("Nothing found");
        return v;
    }
    //todo: use failure here maybe
    pub fn retrieve_data(folder: String, start_name: String) -> bool {
        let img_arg = folder.clone()+ "\\" + &start_name; 

        let img = File::create(folder.clone()+ "\\" + &start_name);
        let xmp = File::create(folder.clone()+ "\\" + &Util::get_XMP_name(&start_name));
        let img_file_path = Path::new(&img_arg);
        let xmp_file_path = Path::new (&(folder+ "\\" + &Util::get_XMP_name(&start_name)));

        if !img_file_path.exists() {
            println!("Image {:?} does not exist", img);
            return false;
        }
        else {
            return true;
        }
        // while img_file_path.exists() && xmp_file_path.exists() {
        //     return true;
        // }
        
    }
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
    pub fn get_exposure(e: ExposeChange) -> i64 {
        return e.expo;
    }
    pub fn set_exposure(mut e: ExposeChange, new_exposure: i64) {
        e.expo = new_exposure;
    }
    pub fn set_increment( e: ExposeChange, c: Change) {
        let tmp = i64::try_from(c.total_image_count).ok().unwrap();
        let increments = e.expo/tmp;
    }

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
    pub fn get_temperature(t: TempChange) -> i32 {
        return t.temperature;
    }
    pub fn set_temperature(mut t: TempChange, new_temperature: i32) {
        t.temperature = new_temperature;
    }
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
    pub fn get_start_list_num(c:Change) -> i32 {
        return c.sln;
    }
    pub fn get_end_list_num(c:Change) -> i32 {
        return c.pln;
    }
    pub fn get_total_images(c:Change) -> i32 {
        return c.total_image_count;
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

impl Util {
    pub fn get_data (file_path: String) {
        let copy = file_path.clone();
        let md = metadata(file_path).unwrap();
        if md.is_file() {
            Util::read_file(copy);
        }
        else {
            //todo: include failure
            warn!("The given argument is not file");
        }
    }
    pub fn read_file(file_path: String) -> String {
        let copy = file_path.clone();
        let md = metadata(file_path).unwrap();
        if md.is_file() == false {
            return String::new();
        } else {
            let contents =
                fs::read_to_string(copy).expect("Something went wrong reading the file");
            return contents;
        }
    }
    pub fn find(path: String, key: String) -> String {
        let data = Util::read_file(path);
        let mut result = String::new();
        let d = "'";
        let mut x = 0;
        let start = data.find(&key).unwrap() + data.len() + 2;

        for c in data.chars() {
            if c.to_string() != d {
                let character = &c.to_string();
                result.push_str(character);
            }
            x += 1;
            if (start + x) > data.len() {
                break;
            }
        }
        return result;
    }
    pub fn replace(path: String, key: String, replace: u64)-> String {
        let data = Util::read_file(path);
        let start = data.find(&key).unwrap() + data.len() + 2;
        let mut end = start;

        let mut result = String::new();
        let d = "'";

        for c in data.chars() {
            if c.to_string() != d {
                result.push_str(&c.to_string());
            };
          end += 1;
        }

          let mut new_data = String::from(&data[0..start]);
          let s = String::from(replace.to_string());
          new_data.push_str(&s);
          new_data.push_str("/");
          let mut_data = &data[end..data.len()];
          new_data.push_str(mut_data);
    
    return new_data.to_string();
}
    pub fn create_value(path:String, pre_key:String, replace:&String, value:String) -> String
    {
        let data = Util::read_file(path);
        let mut start = data.find(&pre_key).unwrap();
        let d = String::new();
        let e = String::new();
        let f = String::from("\"");
        for c in data.chars() {
            if c.to_string() != d {
                start += 1;
            }
        }
        let mut new_file = String::from(&data[0..start]);
        new_file.push_str(replace);
        new_file.push_str(&e);
        new_file.push_str(&f);

        let tmp = &data[start..Some(data.len()).unwrap()];
        new_file.push_str(&tmp);

        return new_file.to_string();
    }

    pub fn get_XMP_name(s:&str) -> String {
        let mut xmp = String::new();
        for k in s.chars() {
            if k != '.' {
                xmp.push(k);
            }
            else {
                xmp.push_str(".xmp");
                break;
            }
        }
        return xmp;
    }

    pub fn is_Numeric(s: String) -> bool {
        for c in s.chars() {
            if !c.is_numeric() {
                return false;
            }
        }
        return true;
    }
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
    pub fn get_xmp(i: Image) ->String {
        return i.xmp_path;
    }
    pub fn get_iso(i: Image) ->i32 {
        return i.iso;
    }
    pub fn get_list(i: Image) -> u32 {
        return i.list;
    }
    pub fn get_white_balance(i: Image) -> u32 {
        return i.white_balance;
    }
    pub fn get_exposure(i: Image) -> u64 {
        return i.exposure;
    }
    pub fn get_shutter_speed(i: Image) -> u64 {
        return i.shutter;
    }
    pub fn get_ratio(i:Image) -> u64 {
        return i.ratio;
    }
    pub fn print_image_info(i: Image) {
        info!("Image file path: {}", i.image_path);
        info!("XMP Path: {}", i.xmp_path);
        info!("White Balance: {}", i.white_balance);
        info!("Exposure: {}", i.exposure);
        info!("Aperture: {}", i.aperture);
        info!("ISO: {}", i.iso);
        info!("Shutter speed: {}", i.shutter);
    }
    
}
