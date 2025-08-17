use std::collections::HashMap;

// The most primitive form of object is basically just a hashmap with values and methods.
fn square_perimeter(square: &HashMap<String, f32>) -> f32 {
    return 4.0 * square.get("side").unwrap();
}

fn square_area(square: &HashMap<String, f32>) -> f32 {
    return square.get("side").unwrap().powf(2.0);
}

fn square_new(_name: String, side: f32) -> HashMap<String, f32> {
    let mut square: HashMap<String, f32> = HashMap::new();

    square.insert("side".to_string(), side);
    square.insert("perimeter".to_string(), square_perimeter(&square));
    square.insert("area".to_string(), square_area(&square));

    return square;
}

fn main() {
    let square_object = square_new("spongebob".to_string(), 20.0);
    println!("perimeter: {}", square_object.get("perimeter").unwrap());
    println!("area: {}", square_object.get("area").unwrap());
}
