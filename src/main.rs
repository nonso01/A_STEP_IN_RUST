// enums vs structs, learning about enums and special Option
#[derive(Debug)]
enum Kingdom {
    Monera(String),
    Protoctista(String),
    Fungi(String),
    Plantae(String),
    Animalia(String),
}

fn main() {
    let human = Kingdom::Animalia(String::from("mammal"));
    println!("human is {:?}", human);
}
