mod codes;
pub mod custom_char;
mod of_node;
pub mod special_char;

use std::fs::{File, OpenOptions};
use std::path::Path;

use std::io::BufWriter;
use std::io::Result;
use std::io::Write;

use codes::SpecialCode;
use codes::WriteInto;

// Increment this number when appropriate:
//
// NUMBER_OF_LCD_SCREENS_DESTROYED_DURING_TESTING: 1

/// A screen that allows you to send commands to a charlcd driver (or whatever
/// that implements the [`Write`] trait).
///
/// # Simple example
///
/// ```no_run
/// extern crate charlcd;
///
/// use charlcd::Screen;
/// use std::io::Write;
///
/// fn main() -> std::io::Result<()> {
///     let mut screen = Screen::default()?; // will use "/dev/lcd" charlcd driver
///
///     screen.clear()?;
///     screen.write(b"hello, world!")?;
///     screen.flash_backlight()?;
///     screen.flush()?; // send all the previous commands to the driver at once
///
///     Ok(())
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
    /// Create a new [`Screen`] instance that will use the provided [`Write`]
    /// under the hood to send commands.
    pub fn new(writer: T) -> Screen<T> {
        Screen { writer }
    }

    /// Clean the rest of the current line, from current cursor position.
    ///
    /// # Live footage (before and after)
    /// ![kill_eol_before](https://crates.microjoe.org/charlcd/media/docs/kill_eol_before.jpg)
    /// ![kill_eol](https://crates.microjoe.org/charlcd/media/docs/kill_eol.jpg)
    ///
    pub fn kill_eol(&mut self) -> std::io::Result<()> {
        write_simple_code!(self, SpecialCode::KillEndOfLine)
    }

    /// Reinitialize the display to its default hardware values.
    ///
    /// # Live footage (before and after)
    /// ![full](https://crates.microjoe.org/charlcd/media/docs/full.jpg)
    /// ![clear](https://crates.microjoe.org/charlcd/media/docs/clear.jpg)
    ///
    /// Note: we observe that the cursor *and* blink are activated after a call
    /// to [`Screen::reinit()`], although the [`Screen::blink_on()`] and
    /// [`Screen::cursor_on()`] methods are *not* called in this function. We
    /// can deduce it is a hardware default to put back the blink and cursor on
    /// at initialization.
    ///
    /// You may want to disable them after a call to this function by using the
    /// [`Screen::blink_off()`] and [`Screen::cursor_off()`] functions.
    pub fn reinit(&mut self) -> std::io::Result<()> {
        write_simple_code!(self, SpecialCode::ReinitializeDisplay)
    }

    /// Disable the display.
    ///
    /// The displayed content is not lost and kept into the screen buffer. Call
    /// [`Screen::display_on()`] to display back what was printed to the screen.
    ///
    /// # Live footage (before and after)
    /// ![full](https://crates.microjoe.org/charlcd/media/docs/full.jpg)
    /// ![display_off](https://crates.microjoe.org/charlcd/media/docs/display_off.jpg)
    ///
    pub fn display_off(&mut self) -> std::io::Result<()> {
        write_simple_code!(self, SpecialCode::DisplayOff)
    }

    /// Enable the display.
    ///
    /// The content of the screen buffer will be displayed back with no change.
    ///
    /// # Live footage (before and after)
    /// ![display_off](https://crates.microjoe.org/charlcd/media/docs/display_off.jpg)
    /// ![full](https://crates.microjoe.org/charlcd/media/docs/full.jpg)
    ///
    pub fn display_on(&mut self) -> std::io::Result<()> {
        write_simple_code!(self, SpecialCode::DisplayOn)
    }

    /// Enable the underscore cursor (independent of blinking cursor).
    ///
    /// # Live footage (before and after)
    /// ![test_clear](https://crates.microjoe.org/charlcd/media/docs/test_clear.jpg)
    /// ![cursor_on](https://crates.microjoe.org/charlcd/media/docs/cursor_on.jpg)
    ///
    pub fn cursor_on(&mut self) -> std::io::Result<()> {
        write_simple_code!(self, SpecialCode::CursorOn)
    }

    /// Disable the underscore cursor (independent of blinking cursor).
    ///
    /// # Live footage (before and after)
    /// ![test](https://crates.microjoe.org/charlcd/media/docs/test.jpg)
    /// ![cursor_off](https://crates.microjoe.org/charlcd/media/docs/blink_on.jpg)
    ///
    pub fn cursor_off(&mut self) -> std::io::Result<()> {
        write_simple_code!(self, SpecialCode::CursorOff)
    }

    /// Enable the blinking cursor (independent of underscore cursor).
    ///
    /// # Live footage (before and after)
    /// ![test_clear](https://crates.microjoe.org/charlcd/media/docs/test_clear.jpg)
    /// ![blink_on](https://crates.microjoe.org/charlcd/media/docs/blink_on.jpg)
    ///
    /// Note: due to long exposure duration of the camera (1 second), the
    /// blinking cursor appears dim in the footage.
    ///
    pub fn blink_on(&mut self) -> std::io::Result<()> {
        write_simple_code!(self, SpecialCode::BlinkOn)
    }

    /// Disable the blinking cursor (independent of underscore cursor).
    ///
    /// # Live footage (before and after)
    /// ![test](https://crates.microjoe.org/charlcd/media/docs/test.jpg)
    /// ![blink_off](https://crates.microjoe.org/charlcd/media/docs/cursor_on.jpg)
    ///
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
    ///
    /// The exact duration is specified in the driver. As of today, the default
    /// value is set to 4 seconds (see the `LCD_BL_TEMPO_PERIOD` define of
    /// the `charlcd.c` driver in your Linux tree).
    pub fn flash_backlight(&mut self) -> std::io::Result<()> {
        write_simple_code!(self, SpecialCode::FlashBacklight)
    }

    /// Clear the screen and return the cursor at original (0, 0) XY position.
    ///
    /// # Live footage (before and after)
    /// ![test](https://crates.microjoe.org/charlcd/media/docs/test.jpg)
    /// ![clear](https://crates.microjoe.org/charlcd/media/docs/clear.jpg)
    ///
    pub fn clear(&mut self) -> std::io::Result<()> {
        self.write(&[0x0c])?; // '\f' escape not defined in Rust
        Ok(())
    }

    /// Move the cursor back one character, and delete the character at this
    /// position.
    ///
    /// This is an utility function that will send the raw byte value for the
    /// `'\b'` escape sequence. This sequence is valid in C, but is not
    /// available in Rust.
    ///
    /// # Live footage (before and after)
    /// ![test](https://crates.microjoe.org/charlcd/media/docs/test.jpg)
    /// ![back](https://crates.microjoe.org/charlcd/media/docs/back.jpg)
    ///
    pub fn back(&mut self) -> std::io::Result<()> {
        self.write(&[0x08])?; // '\b' escape not defined in Rust
        Ok(())
    }

    // Less-used (and some non-working?) methods below

    /// Shift cursor left.
    ///
    /// # Live footage (before and after)
    /// ![test](https://crates.microjoe.org/charlcd/media/docs/test.jpg)
    /// ![shift_cursor_left](https://crates.microjoe.org/charlcd/media/docs/shift_cursor_left.jpg)
    ///
    pub fn shift_cursor_left(&mut self) -> std::io::Result<()> {
        write_simple_code!(self, SpecialCode::ShiftCursorLeft)
    }

    /// Shift cursor right.
    ///
    /// # Live footage (before and after)
    /// ![test](https://crates.microjoe.org/charlcd/media/docs/test.jpg)
    /// ![shift_cursor_right](https://crates.microjoe.org/charlcd/media/docs/shift_cursor_right.jpg)
    ///
    pub fn shift_cursor_right(&mut self) -> std::io::Result<()> {
        write_simple_code!(self, SpecialCode::ShiftCursorRight)
    }

    /// Shift display left.
    ///
    /// # Live footage (before and after)
    /// ![shift](https://crates.microjoe.org/charlcd/media/docs/shift.jpg)
    /// ![shift_display_left](https://crates.microjoe.org/charlcd/media/docs/shift_display_left.jpg)
    ///
    /// Note: we can observe that the shift will create an artefact on the n+2
    /// line, as the extra characters will be shifted there.
    pub fn shift_display_left(&mut self) -> std::io::Result<()> {
        write_simple_code!(self, SpecialCode::ShiftDisplayLeft)
    }

    /// Shift display right.
    ///
    /// # Live footage (before and after)
    /// ![shift](https://crates.microjoe.org/charlcd/media/docs/shift.jpg)
    /// ![shift_display_right](https://crates.microjoe.org/charlcd/media/docs/shift_display_right.jpg)
    ///
    /// Note: we can observe that the shift will create an artefact on the n+2
    /// line, as the extra characters will be shifted there.
    pub fn shift_display_right(&mut self) -> std::io::Result<()> {
        write_simple_code!(self, SpecialCode::ShiftDisplayRight)
    }

    /// Enable one line mode.
    ///
    /// # Live footage (before and after)
    /// ![full](https://crates.microjoe.org/charlcd/media/docs/full.jpg)
    /// ![one_line](https://crates.microjoe.org/charlcd/media/docs/one_line.jpg)
    ///
    /// We can see that the screen seems to disable power for the second and
    /// fourth line of the display (in case of a 4 lines one). Cutting the
    /// power for half the screen means that the contrast adjustment will not
    /// be correct anymore, as the screen uses less power by managing only half
    /// of the characters.
    ///
    /// A manual recalibration of the contrast will be necessary if you change
    /// between [`Screen::one_line()`] and [`Screen::two_lines()`] modes.
    pub fn one_line(&mut self) -> std::io::Result<()> {
        write_simple_code!(self, SpecialCode::OneLine)
    }

    /// Enable two lines mode.
    ///
    /// # Live footage (before and after)
    /// ![one_line](https://crates.microjoe.org/charlcd/media/docs/one_line.jpg)
    /// ![two_lines](https://crates.microjoe.org/charlcd/media/docs/two_lines.jpg)
    ///
    /// This will mess up the screen if coming from [`Screen::one_line()`] mode.
    ///
    /// A manual recalibration of the contrast will be necessary if you change
    /// between [`Screen::one_line()`] and [`Screen::two_lines()`] modes.
    pub fn two_lines(&mut self) -> std::io::Result<()> {
        write_simple_code!(self, SpecialCode::TwoLines)
    }

    /// Enable small font mode.
    ///
    /// Note: this function seems to have no effect on the screen after tests
    /// with multiple screen variants. No relevant footage available.
    pub fn small_font(&mut self) -> std::io::Result<()> {
        write_simple_code!(self, SpecialCode::SmallFont)
    }

    /// Enable big font mode.
    ///
    /// Note: this function seems to have no effect on the screen after tests
    /// with multiple screen variants. No relevant footage available.
    pub fn large_font(&mut self) -> std::io::Result<()> {
        write_simple_code!(self, SpecialCode::LargeFont)
    }

    /// Custom character create
    pub fn custom_char(&mut self, code: u8, value: [u8; 8]) -> std::io::Result<()> {
        let mut res = 0u64;
        let mut i = 0;
        for b in value.iter().rev() {
            res |= (*b as u64) << i;
            i += 8;
        }
        write_simple_code!(self, SpecialCode::Generator(code, res))
    }

    pub fn gotoxy(&mut self, x: u32, y: u32) -> std::io::Result<()> {
        write_simple_code!(self, SpecialCode::GotoXY(Some(x), Some(y)))
    }

    pub fn gotox(&mut self, x: u32) -> std::io::Result<()> {
        write_simple_code!(self, SpecialCode::GotoXY(Some(x), None))
    }

    pub fn gotoy(&mut self, y: u32) -> std::io::Result<()> {
        write_simple_code!(self, SpecialCode::GotoXY(None, Some(y)))
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

type FileScreen = Screen<BufWriter<File>>;

const DEFAULT_SCREEN_DEV_PATH: &str = "/dev/lcd";

impl FileScreen {
    /// Create a Screen instance based on the passed path to the device.
    pub fn from_dev_path(path: &Path) -> std::io::Result<FileScreen> {
        let file = OpenOptions::new().write(true).open(path)?;
        let buf = BufWriter::new(file);
        Ok(Screen::new(buf))
    }

    /// Create a default Screen instance based on `/dev/lcd` device driver
    /// path.
    pub fn default() -> std::io::Result<FileScreen> {
        Screen::from_dev_path(&Path::new(DEFAULT_SCREEN_DEV_PATH))
    }

    /// Get the width of the screen, in number of characters it can display.
    ///
    /// # Example
    ///
    /// ```no_run
    /// extern crate charlcd;
    ///
    /// use charlcd::Screen;
    ///
    /// fn main() -> std::io::Result<()> {
    ///     let screen = Screen::default()?; // the screen is 20x4 in this test
    ///
    ///     let width = screen.width()?;
    ///     assert_eq!(width, 20);
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Important note
    ///
    /// The implementation behind this function is currently a hack that will
    /// go find the value in the `auxdisplay` platform device tree node in
    /// `/sys/devices/platform/auxdisplay/of_node/*`. This is because the
    /// `charlcd` driver does not export the `width` nor `height` fields to
    /// userspace.
    ///
    /// In the future, this function may be able to read the value directly
    /// from the `/dev/lcd` device if a proper `ioctl` or `read` call is
    /// implemented for this purpose.
    ///
    pub fn width(&self) -> std::io::Result<u32> {
        of_node::display_width_chars()
    }

    /// Get the height of the screen, in number of characters it can display.
    ///
    /// # Example
    ///
    /// ```no_run
    /// extern crate charlcd;
    ///
    /// use charlcd::Screen;
    ///
    /// fn main() -> std::io::Result<()> {
    ///     let screen = Screen::default()?; // the screen is 20x4 in this test
    ///
    ///     let height = screen.height()?;
    ///     assert_eq!(height, 4);
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Important note
    ///
    /// The implementation behind this function is currently a hack that will
    /// go find the value in the `auxdisplay` platform device tree node in
    /// `/sys/devices/platform/auxdisplay/of_node/*`. This is because the
    /// `charlcd` driver does not export the `width` nor `height` fields to
    /// userspace.
    ///
    /// In the future, this function may be able to read the value directly
    /// from the `/dev/lcd` device if a proper `ioctl` or `read` call is
    /// implemented for this purpose.
    ///
    pub fn height(&self) -> std::io::Result<u32> {
        of_node::display_height_chars()
    }
}
