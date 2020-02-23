use charlcd::special_char;
use charlcd::Screen;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let mut screen = Screen::default()?;

    screen.clear()?;

    screen.write(&[special_char::ALPHA])?;
    screen.write(&[special_char::BETA])?;
    screen.write(&[special_char::EPSILON])?;
    screen.write(&[special_char::MU])?;
    screen.write(&[special_char::SIGMA])?;
    screen.write(&[special_char::RO])?;
    screen.write(&[special_char::THETA])?;
    screen.write(&[special_char::OMEGA])?;
    screen.write(&[special_char::SIGMA_UPPER])?;
    screen.write(&[special_char::PI])?;

    screen.write(b"\n")?;

    screen.write(&[special_char::SQRT])?;
    screen.write(&[special_char::INV])?;
    screen.write(&[special_char::INFINITE])?;
    screen.write(&[special_char::DIV])?;
    screen.write(&[special_char::MEAN])?;

    screen.write(&[special_char::MEDIAN_DOT])?;
    screen.write(&[special_char::BLOCK])?;

    screen.flush()?;

    Ok(())
}
