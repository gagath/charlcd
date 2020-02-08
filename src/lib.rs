mod codes;
mod of_node;

use std::fs::{File, OpenOptions};
use std::path::Path;

use std::io::BufWriter;
use std::io::Result;
use std::io::Write;

use codes::SpecialCode;
use codes::WriteInto;

/// A screen that allows you to send commands to a charlcd driver (or whatever
/// that implements the `Write` trait).
///
/// Nominal usage:
///
/// ```rust
/// use charlcd::Screen;
///
/// fn main() -> std::io::Result<()> {
///     let mut screen = Screen::default(); // will use "/dev/lcd" charlcd driver
///
///     screen.clear()?;
///     screen.write(b"hello, world!")?;
///     screen.flash_backlight()?;
///     screen.flush()?; // send all the previous commands to the driver at once
/// }
/// ```
pub struct Screen<T> {
    writer: T,
}

macro_rules! write_simple_code {
    ($self:expr, $code:expr) => {{
        $code.write_into(&mut $self.writer)?;
        Ok(())
    }};
}

impl<T> Screen<T>
where
    T: Write,
{
    /// Create a new display instance that will use the provided Writer under
    /// the hood to send commands.
    pub fn new(writer: T) -> Screen<T> {
        Screen { writer }
    }

    /// Clean the rest of the current line, from cursor's position.
    pub fn kill_eol(&mut self) -> std::io::Result<()> {
        write_simple_code!(self, SpecialCode::KillEndOfLine)
    }

    /// Reinitialize the display to its default values.
    pub fn reinit(&mut self) -> std::io::Result<()> {
        write_simple_code!(self, SpecialCode::ReinitializeDisplay)
    }

    /// Enable the display text output.
    pub fn display_on(&mut self) -> std::io::Result<()> {
        write_simple_code!(self, SpecialCode::DisplayOn)
    }

    /// Disable the display text output.
    pub fn display_off(&mut self) -> std::io::Result<()> {
        write_simple_code!(self, SpecialCode::DisplayOff)
    }

    /// Enable the underscore cursor (independent of blinking cursor).
    pub fn cursor_on(&mut self) -> std::io::Result<()> {
        write_simple_code!(self, SpecialCode::CursorOn)
    }

    /// Disable the underscore cursor (independent of blinking cursor).
    pub fn cursor_off(&mut self) -> std::io::Result<()> {
        write_simple_code!(self, SpecialCode::CursorOff)
    }

    /// Enable the blinking cursor (independent of underscore cursor).
    pub fn blink_on(&mut self) -> std::io::Result<()> {
        write_simple_code!(self, SpecialCode::BlinkOn)
    }

    /// Disable the blinking cursor (independent of underscore cursor).
    pub fn blink_off(&mut self) -> std::io::Result<()> {
        write_simple_code!(self, SpecialCode::BlinkOff)
    }

    /// Enable the backlight.
    pub fn backlight_on(&mut self) -> std::io::Result<()> {
        write_simple_code!(self, SpecialCode::BacklightOn)
    }

    /// Disable the backlight.
    pub fn backlight_off(&mut self) -> std::io::Result<()> {
        write_simple_code!(self, SpecialCode::BacklightOff)
    }

    /// Flash the backlight during a small duration.
    pub fn flash_backlight(&mut self) -> std::io::Result<()> {
        write_simple_code!(self, SpecialCode::FlashBacklight)
    }

    /// Clear the screen and return the cursor at original (0, 0) XY position.
    pub fn clear(&mut self) -> std::io::Result<()> {
        self.write(&[0x0c])?; // '\f' escape not defined in Rust
        Ok(())
    }

    /// Move the cursor back one character.
    pub fn back(&mut self) -> std::io::Result<()> {
        self.write(&[0x08])?; // '\b' escape not defined in Rust
        Ok(())
    }

    // Less-used (and some non-working?) methods below

    /// Shift cursor left.
    pub fn shift_cursor_left(&mut self) -> std::io::Result<()> {
        write_simple_code!(self, SpecialCode::ShiftCursorLeft)
    }

    /// Shift cursor right.
    pub fn shift_cursor_right(&mut self) -> std::io::Result<()> {
        write_simple_code!(self, SpecialCode::ShiftCursorRight)
    }

    /// Shift display left.
    pub fn shift_display_left(&mut self) -> std::io::Result<()> {
        write_simple_code!(self, SpecialCode::ShiftDisplayLeft)
    }

    /// Shift display right.
    pub fn shift_display_right(&mut self) -> std::io::Result<()> {
        write_simple_code!(self, SpecialCode::ShiftDisplayRight)
    }

    /// Enable one line mode.
    ///
    /// ![test](https://blog.microjoe.org/images/hd44780-lcd-i2c-screen-using-linux-mainline-charlcd-driver/linux_419.medium.jpg)
    pub fn one_line(&mut self) -> std::io::Result<()> {
        write_simple_code!(self, SpecialCode::OneLine)
    }

    /// Enable two lines mode.
    pub fn two_lines(&mut self) -> std::io::Result<()> {
        write_simple_code!(self, SpecialCode::TwoLines)
    }

    /// Enable small font mode.
    ///
    /// Seems to have no effect on the screen given the tests with multiple
    /// screen variants.
    pub fn small_font(&mut self) -> std::io::Result<()> {
        write_simple_code!(self, SpecialCode::SmallFont)
    }

    /// Enable big font mode.
    ///
    /// Seems to have no effect on the screen given the tests with multiple
    /// screen variants.
    pub fn large_font(&mut self) -> std::io::Result<()> {
        write_simple_code!(self, SpecialCode::LargeFont)
    }
}

// Reimplement Write trait for Screen, so that user can call the write and
// flush methods of the inner writer.
impl<T> Write for Screen<T>
where
    T: Write,
{
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.writer.write(buf)
    }
    fn flush(&mut self) -> Result<()> {
        self.writer.flush()
    }
}

// Concrete screen based on a File, to write to the real charlcd driver (or to
// another file).
type FileScreen = Screen<BufWriter<File>>;

const DEFAULT_SCREEN_DEV_PATH: &str = "/dev/lcd";

impl FileScreen {
    /// Create a Screen instance based on the passed path to the device.
    pub fn from_dev_path(path: &Path) -> std::io::Result<FileScreen> {
        let file = OpenOptions::new().write(true).open(path)?;
        let buf = BufWriter::new(file);
        Ok(Screen::new(buf))
    }

    /// Create a default Screen instance based on `"/dev/lcd"` path.
    pub fn default() -> std::io::Result<FileScreen> {
        Screen::from_dev_path(&Path::new(DEFAULT_SCREEN_DEV_PATH))
    }

    /// Get the width of the screen, in number of characters it can display.
    ///
    /// **Important note:** The implementation behind this function is
    /// currently a hack that will go find the value in the `auxdisplay`
    /// platform device tree node in
    /// `/sys/devices/platform/auxdisplay/of_node/*`. This is because the
    /// `charlcd` driver does not export the screen width nor height to
    /// userspace (could be using `ioctl` or `read` syscalls).
    ///
    pub fn width(&self) -> std::io::Result<u32> {
        of_node::display_width_chars()
    }

    /// Get the height of the screen, in number of characters it can display.
    ///
    /// **Important note:** The implementation behind this function is
    /// currently a hack that will go find the value in the `auxdisplay`
    /// platform device tree node in
    /// `/sys/devices/platform/auxdisplay/of_node/*`. This is because the
    /// `charlcd` driver does not export the screen width nor height to
    /// userspace (could be using `ioctl` or `read` syscalls).
    ///
    pub fn height(&self) -> std::io::Result<u32> {
        of_node::display_height_chars()
    }
}
