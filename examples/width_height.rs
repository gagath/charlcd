use charlcd::Screen;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let mut screen = Screen::default()?;

    screen.clear()?;
    screen.write(format!("width: {}\n", screen.width()?).as_bytes())?;
    screen.write(format!("height: {}\n", screen.height()?).as_bytes())?;

    Ok(())
}
