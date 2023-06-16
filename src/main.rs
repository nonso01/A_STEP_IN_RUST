// constants, some might be definded already
const PI: f64 = 3.142857;
const ONE_POINT_THREE: f64 = 1.333333;

// struct basics

#[derive(Debug)]
struct Sphere {
    radius: f64,
   // width: f64,
   // height: f64,
}

impl Sphere {
    fn volume(&self) -> f64 {
        ONE_POINT_THREE * PI * self.radius * self.radius * self.radius
    }

    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }
}

fn main() {
    let sphere1 = Sphere {
        radius: 4.5,
       // height: 10.0,
      //  width: 5.0,
    };
    
    println!("the radius of sphere1 is {:?} and area is {:?}",
             sphere1.volume(),
             sphere1.area());
}


