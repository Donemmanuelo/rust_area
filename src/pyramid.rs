use crate::traits::{area::Area, form_data::FormData};
use std::io::{self, Error, ErrorKind
};
#[derive(Debug)]
pub struct Pyramid {
    radius: f64,
    length: f64,
}
  

impl Area for Pyramid {
    fn area(&self) -> f64{
       4.0 * self.radius.powf(3.0) +  self.length.powf(2.0)
    }
}
impl Pyramid {
    pub  fn from(radius: f64, length: f64) -> Self {
        Self {radius, length}
    }

pub fn get_radius(&self) -> f64 {
    self.radius
}
pub fn set_radius(&mut self, radius: f64) {
    self.radius = radius;
}
pub fn get_length(&self) -> f64 {
    self.length
}
pub fn set_length(&mut self, length: f64) {
    self.length = length;
}
}

impl FormData for Pyramid{
    fn collect_data(&mut self) -> Result<(), Error> {
        println!("Enter the Pyramid radius: ");
        let mut radius = String::new();

        io::stdin().read_line(&mut radius)?;
        let radius = match radius.trim().parse() {
            Ok(value) => value,
            Err(_) => {
                //Returns a error of kind invalid input
                return Err(Error::new(ErrorKind::InvalidInput, "oh! sorry, please input a number"))
            }
        };
        println!("Enter the Pyramid length: ");
        let mut length = String::new();

        io::stdin().read_line(&mut length)?;
        let length = match length.trim().parse() {
            Ok(value) => value,
            Err(_) => {
                //Returns a error of kind invalid input
                return Err(Error::new(ErrorKind::InvalidInput, "oh! sorry, please input a number"))
            }
        };
        self.length = length;
        self.radius = radius;
        Ok(())
    }

    fn new() -> Self  {
        Self { radius: 0.0, length: 0.0}
    }
}
