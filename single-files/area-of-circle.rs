
use std::io::Write;

fn get_area_of_circle(r: f32) -> f32 {
    let pi = std::f32::consts::PI;
    return 2.0 * pi * pi * r;
}

fn get_input() -> f32 {
    print!("radius: ");
    std::io::stdout().flush().unwrap();
    let mut input: String = String::new();
    std::io::stdin().read_line(&mut input)
        .expect("error while reading the input");
    let num: f32 = input.trim().parse().unwrap_or(0.0);
    print!("entered radius: {}", num);
    num
}

fn print_area(area: f32) {
    print!("area of circle = {:.2} meters", area);
}

fn main() {
    let radius = get_input(); // in meters
    let area: f32 = get_area_of_circle(radius);
    print_area(area);
}







