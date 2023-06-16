// struct basics including method syntax lol, looks like if am writing js

#[derive(Debug)]
struct BoxShape {
    width: u32,
    height: u32,
    length: u32,
}

impl BoxShape {
   fn volume(&self) -> u32 {
        self.width * self.height * self.length
    }

   fn area(self: &Self) -> u32 {
       self.width * self.height
   }
}

fn main() {
    let box1 = BoxShape {
        width: 30,
        length: 40,
        height: 30,
    };

    println!(
        "the volume of box1 is {:?}L and area is {:?}",
        box1.volume(),
        box1.area()
        );
}
    
