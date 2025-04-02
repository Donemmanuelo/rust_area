use crate::traits::{area::Area, form_data::FormData};
use std::{
    f64::consts::PI, 
    io::{self, Error, ErrorKind},
};
#[derive(Debug)]
pub struct Cylinder {
    radius: f64,
    height: f64,
}
  

impl Area for Cylinder {
    fn area(&self) -> f64{
        2.0 * self.radius * PI * self.height + 2.0 * self.radius.powf(2.0) * PI
    }
}
impl Cylinder {
    pub fn from(height: f64, radius: f64) -> Self {
        Self {height, radius}
    }
    pub fn get_radius(&self) -> f64 {
        self.radius
    }
    pub fn get_height(&self) -> f64 {
        self.height
    }
        


    pub fn set_height(&mut self, height:f64)  {
        self.height = height;
    }
    pub fn set_radius(&mut self, radius: f64)  {
        self.radius= radius;
    }

}
impl FormData for Cylinder {
    fn collect_data(&mut self) -> Result<(), Error> {
        println!("Enter the cylinder height");
        let mut height = String::new();

        io::stdin().read_line(&mut height)?;
        let height = match height.trim().parse() {
            Ok(value) => value,
            Err(_) => {
                //Returns a error of kind invalid input
                return Err(Error::new(ErrorKind::InvalidInput, "oh! sorry, please input a number"))
            }
            
        };     
    
        println!("Enter the cylinder radius ");
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
        self.height = height;
        Ok(())
    }

    fn new() -> Self {
        Self { height: 0.0, radius: 0.0}
    }
    }