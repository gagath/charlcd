use std::io::Write;

use std::{thread, time};

use charlcd::Screen;

macro_rules! test_method {
    ($screen: ident, $method:ident) => {
        $screen.clear()?;
        $screen.flush()?;
        $screen.write(stringify!($method).as_bytes())?;
        $screen.write(b"..")?;
        $screen.flush()?;

        thread::sleep(time::Duration::from_secs(2));

        $screen.$method()?;
        $screen.write(b"ok")?;
        $screen.flush()?;

        thread::sleep(time::Duration::from_secs(2));
    };
}

fn main() -> std::io::Result<()> {
    let mut screen = Screen::default()?;

    test_method!(screen, reinit);
    test_method!(screen, display_off);
    test_method!(screen, display_on);
    test_method!(screen, backlight_on);
    test_method!(screen, backlight_off);
    test_method!(screen, cursor_off);
    test_method!(screen, cursor_on);
    test_method!(screen, blink_on);
    test_method!(screen, blink_off);
    test_method!(screen, shift_cursor_left);
    test_method!(screen, shift_cursor_right);
    test_method!(screen, shift_display_left);
    test_method!(screen, kill_eol);
    test_method!(screen, shift_display_right);
    test_method!(screen, one_line);
    test_method!(screen, two_lines);

    // reinit everything back after the test
    test_method!(screen, reinit);

    Ok(())
}
