fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _context = sdl2::init()?;

    println!("Hello, world!");

    Ok(())
}
