// the rust way of doing this is to use traits (basically interface in other languages).
trait Shape {
    fn name(&self) -> &str;
    fn area(&self) -> f32;
    fn perimeter(&self) -> f32;
}

struct Square {
    name: String,
    side: f32,
}

impl Square {
    pub fn new(name: String, side: f32) -> Self {
        Self { name, side }
    }
}

impl Shape for Square {
    fn name(&self) -> &str {
        return &self.name;
    }

    fn area(&self) -> f32 {
        return self.side.powf(2.0);
    }

    fn perimeter(&self) -> f32 {
        return 4.0 * self.side;
    }
}

fn main() {
    let square = Square::new(String::from("Squarepants"), 4.0);
    println!("name: {}", square.name());
    println!("area: {}", square.area());
    println!("square: {}", square.perimeter());
}
