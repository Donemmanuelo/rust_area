use crate::traits::{area::Area, form_data::FormData};
use std::io::{self, Error, ErrorKind
};
#[derive(Debug)]
pub struct Rectangle {
     length: f64,
     width: f64,
}



impl Area for Rectangle {
    fn area(&self) -> f64 {
        self.length * self.width
    }
}

impl Rectangle {
    pub fn from(width: f64, length: f64) -> Self {
        Self {width, length}
    }
    pub fn get_width(&self) -> f64 {
        self.width
    }
    pub fn get_length(&self) -> f64 {
        self.length
    }
   

    pub fn set_width(&mut self, width:f64)  {
        self.width = width;
    }
    pub fn set_length(&mut self, length: f64)  {
        self.length = length;
    }
}
impl FormData for Rectangle{
    fn collect_data(&mut self) -> Result<(), Error> {
        println!("Enter the rectangle length: ");
        let mut length = String::new();

        io::stdin().read_line(&mut length)?;
        let length = match length.trim().parse() {
            Ok(value) => value,
            Err(_) => {
                //Returns a error of kind invalid input
                return Err(Error::new(ErrorKind::InvalidInput, "oh! sorry, please input a number"))
            }
        };
       
    
        println!("Enter the rectangle width ");
        let mut width = String::new();

        io::stdin().read_line(&mut width)?;
        let width = match width.trim().parse() {
            Ok(value) => value,
            Err(_) => {
                //Returns a error of kind invalid input
                return Err(Error::new(ErrorKind::InvalidInput, "oh! sorry, please input a number"))
            }
        };
        self.width = width;
        self.length = length;
        Ok(())
    }

    fn new() -> Self {
        Self { length: 0.0, width: 0.0}
    }
    }
   

