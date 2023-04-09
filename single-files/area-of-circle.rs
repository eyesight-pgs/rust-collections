

fn get_area_of_circle(r: f32) -> f32 {
    let pi = std::f32::consts::PI;
    return 2.0 * pi * pi * r;
}


fn main() {
    let radius = 10.0; // in meters
    let area: f32 = get_area_of_circle(radius);
    print!("area of circle = {} meters", area);
}







