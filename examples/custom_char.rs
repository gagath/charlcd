use charlcd::custom_char;
use charlcd::Screen;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let mut screen = Screen::default()?;

    // Alarm symbol
    screen.custom_char(0, [0x04, 0x0E, 0x0E, 0x0E, 0x0E, 0x1F, 0x04, 0x00])?;

    screen.custom_char(1, custom_char::RIGHT_TRIANGLE)?;
    screen.custom_char(2, custom_char::LEFT_TRIANGLE)?;
    screen.custom_char(3, custom_char::UP_TRIANGLE)?;
    screen.custom_char(4, custom_char::DOWN_TRIANGLE)?;

    screen.clear()?;
    screen.write(b"\x00 Alarm 10:00\n")?;
    screen.write(b"\x01\x02 X Arrows\n")?;
    screen.write(b"\x03\x04 Y Arrow")?;
    screen.flush()?;

    Ok(())
}
