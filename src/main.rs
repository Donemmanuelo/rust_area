use crate::traits::area::Area;
use crate::traits::form_data::FormData;
use circle::Circle;
use rectan::Rectangle;
use std::io;
use cylinder::Cylinder;
use pyramid::Pyramid;
use sphere::Sphere;
use hemisphere::Hemisphere;

fn main() {
    let mut circ = Circle::new();
    let mut rectangle = Rectangle::new();
    let mut cylinder = Cylinder::new();
    let mut pyramid = Pyramid::new();
    let mut sphere = Sphere::new();
    let mut hemisphere = Hemisphere::new();
   println!("Do you want to calculate the area of Circle or rectangle/1.Circle, 2.rectangle, 3.cylinder, 4.Pyramid, 5.sphere, 6.hemisphere/");
   let  mut string = String::new();
   //let string =  string.as_str();
   io::stdin()
       .read_line(&mut string)
       .expect("failed to read line");
let string: char = match string.trim().parse() {
    Ok(u8) => u8,
    Err(e) => return println!(" {}", e),
};

//let t = String::from("Circle");
// let string = string.parse();
//let u = String::from("rectangle");

    if '1' == string {
        loop {
            
            //read user input and supply it to Circle
            let result = circ.collect_data();
            if result.is_ok() {
                break;
            }
            eprintln!(":{:?}", result);
        }
        println!("{:?} area: {}", circ, circ.area());
   }
   else if '2' == string {
        loop {
            let result1 = rectangle.collect_data();
            if result1.is_ok() {
                break;
            }
            eprintln!(":{:?}", result1);
        }
        println!("{:?} area: {}", rectangle, rectangle.area());
    }
    else if '3' == string {
            loop {
                let result2 = cylinder.collect_data();
                if result2.is_ok() {
                    break;
                }
                eprintln!(":{:?}", result2);
            }
            println!("{:?} area: {}", cylinder, cylinder.area());
    }
    else if '4' == string {
        loop {
            let result = pyramid.collect_data();
            if result.is_ok() {
                break;
            }
            eprintln!(":{:?}", result);
        }
        println!("{:?} area: {}", pyramid, pyramid.area());
}
else if '5' == string {
    loop {
        let result = sphere.collect_data();
        if result.is_ok() {
            break;
        }
        eprintln!(":{:?}", result);
    }
    println!("{:?} area: {}", sphere, sphere.area());
}
else if '6' == string {
    loop {
        let result = hemisphere.collect_data();
        if result.is_ok() {
            break;
        }
        eprintln!(":{:?}", result);
    }
    println!("{:?} area: {}", hemisphere, hemisphere.area());
}
}

mod circle;
mod rectan;
mod traits;
mod cylinder;
mod pyramid;
mod sphere;
mod hemisphere;