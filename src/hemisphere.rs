use crate::traits::{area::Area, form_data::FormData};
use std::{
    f64::consts::PI, 
    io::{self, Error, ErrorKind},
};
#[derive(Debug)]
pub struct Hemisphere{
    radius: f64
}
  

impl Area for Hemisphere{
    fn area(&self) -> f64{
        4.0 *self.radius.powf(2.0) * PI / 2.0
    }
}


impl Hemisphere{
 
pub fn get_radius(&self) -> f64 {
    self.radius
}
pub fn set_radius(&mut self, radius: f64) {
    self.radius = radius;
}
}

impl FormData for Hemisphere{
    fn collect_data(&mut self) -> Result<(), Error> {
        println!("Enter the Hemisphere radius: ");
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
