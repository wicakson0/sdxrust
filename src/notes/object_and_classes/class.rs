// the next iteration in the object is the class, it enforces a certain structure for instances.
struct Square {
    name: String,
    side: f32,
}

impl Square {
    pub fn new(name: String, side: f32) -> Self {
        Self { name, side }
    }

    pub fn perimeter(&self) -> f32 {
        return 4.0 * self.side;
    }

    pub fn area(&self) -> f32 {
        return self.side.powf(2.0);
    }
}

fn main() {
    let square = Square::new(String::from("spongebob"), 5.0);
    println!("name: {}", square.name);
    println!("perimeter: {}", square.perimeter());
    println!("area: {}", square.area());
}
