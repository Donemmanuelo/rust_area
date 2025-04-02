use crate::traits::{area::Area, form_data::FormData};
use std::{
    f64::consts::PI, 
    io::{self, Error, ErrorKind},
};
#[derive(Debug)]
pub struct Circle {
    radius: f64
}
  

impl Area for Circle {
    fn area(&self) -> f64{
        self.radius.powf(2.0) * PI
    }
}
impl Circle {
    pub  fn from(radius: f64) -> Self {
        Self {radius}
    }

    

pub fn get_radius(&self) -> f64 {
    self.radius
}
pub fn set_radius(&mut self, radius: f64) {
    self.radius = radius;
}
}

impl FormData for Circle{
    fn collect_data(&mut self) -> Result<(), Error> {
        println!("Enter the Circle radius: ");
        let mut radius = String::new();

        io::stdin().read_line(&mut radius)?;
        let radius = match radius.trim().parse() {
            Ok(value) => value,
            Err(_) => {
                //Returns a error of kind invalid input
                return Err(Error::new(ErrorKind::InvalidInput, "oh! sorry, please input a number"))
            }
        };
        self.radius = radius;
        Ok(())
    }

    fn new() -> Self  {
        Self { radius: 0.0}
    }
}
