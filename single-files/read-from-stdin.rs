use std::io::Write;


fn main() -> std::io::Result<()> {
    print!("enter radius: ");
    std::io::stdout().flush()?;
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("error while reading the input");
    let radius: f32 = input.trim().parse().unwrap_or(0.0);
    print!("entered value: {} meters", radius);
    Ok(())
}











