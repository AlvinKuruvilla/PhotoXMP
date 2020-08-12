extern crate log;

use std::fs::metadata;
use log::{warn};
use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write, BufRead, Read};

use crate::Image;

#[derive(Debug)]
pub struct Util;

impl Util {
    pub fn write_file(i: Image, new_data: String) {
        let mut file = File::create(i.get_xmp().unwrap()).expect("Unable to create requested file");

        let mut bw = BufWriter::new(file);
        
        bw.write_all(new_data.as_bytes()).expect("Unable to write to file");
    }
    pub fn write_file_from_ref(i: &Image, new_data: String) {
        let mut file = File::create(i.get_xmp().unwrap()).expect("Unable to create requested file");

        let mut bw = BufWriter::new(file);

        bw.write_all(new_data.as_bytes()).expect("Unable to write to file");
    }

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
    //todo: use BufRead
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