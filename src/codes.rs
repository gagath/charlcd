use std::io::{Error, ErrorKind, Result, Write};

const ESCAPE_CODE: &[u8] = "\x1b[L".as_bytes();
const GENERATOR_MAX_CHAR_INDEX: u8 = 7;

macro_rules! write_char {
    ($writer:ident, $char:expr) => {
        $writer.write(&[$char as u8])
    };
}

macro_rules! write_digit {
    ($writer:ident, $num:expr) => {
        $writer.write(&[$num + '0' as u8])
    };
}

pub trait WriteInto<W>
where
    W: Write,
{
    fn write_into(self, writer: &mut W) -> Result<usize>;
}

pub enum SpecialCode {
    DisplayOn,
    DisplayOff,
    CursorOn,
    CursorOff,
    BlinkOn,
    BlinkOff,
    BacklightOn,
    BacklightOff,
    FlashBacklight,
    SmallFont,
    LargeFont,
    OneLine,
    TwoLines,
    ShiftCursorLeft,
    ShiftCursorRight,
    ShiftDisplayLeft,
    ShiftDisplayRight,
    KillEndOfLine,
    ReinitializeDisplay,
    Generator(u8, u64),
    GotoXY(Option<u32>, Option<u32>),
}

fn write_goto_xy_sym<T: Write>(writer: &mut T, sym: char, value: Option<u32>) -> Result<usize> {
    let mut total = 0;
    if let Some(value) = value {
        total += write_char!(writer, sym)?;
        total += writer.write(value.to_string().as_bytes())?;
    }
    Ok(total)
}

fn write_goto_xy<T: Write>(writer: &mut T, x: Option<u32>, y: Option<u32>) -> Result<usize> {
    let mut total = 0;
    total += write_goto_xy_sym(writer, 'x', x)?;
    total += write_goto_xy_sym(writer, 'y', y)?;
    total += write_char!(writer, ';')?;
    Ok(total)
}

fn write_generator<T: Write>(writer: &mut T, char_index: u8, value: u64) -> Result<usize> {
    if char_index > GENERATOR_MAX_CHAR_INDEX {
        return Err(Error::new(
            ErrorKind::Other,
            format!(
                "char index cannot be greater than {}",
                GENERATOR_MAX_CHAR_INDEX
            ),
        ));
    }

    let mut total = 0;
    total += write_char!(writer, 'G')?;
    total += write_digit!(writer, char_index)?;
    total += writer.write(format!("{:>016x}", value).as_bytes())?;
    total += write_char!(writer, ';')?;
    Ok(total)
}

impl<W> WriteInto<W> for SpecialCode
where
    W: Write,
{
    fn write_into(self, writer: &mut W) -> Result<usize> {
        let mut total = 0;

        total += writer.write(ESCAPE_CODE)?;
        total += match self {
            SpecialCode::DisplayOn => write_char!(writer, 'D')?,
            SpecialCode::DisplayOff => write_char!(writer, 'd')?,
            SpecialCode::CursorOn => write_char!(writer, 'C')?,
            SpecialCode::CursorOff => write_char!(writer, 'c')?,
            SpecialCode::BlinkOn => write_char!(writer, 'B')?,
            SpecialCode::BlinkOff => write_char!(writer, 'b')?,
            SpecialCode::BacklightOn => write_char!(writer, '+')?,
            SpecialCode::BacklightOff => write_char!(writer, '-')?,
            SpecialCode::FlashBacklight => write_char!(writer, '*')?,
            SpecialCode::SmallFont => write_char!(writer, 'f')?,
            SpecialCode::LargeFont => write_char!(writer, 'F')?,
            SpecialCode::OneLine => write_char!(writer, 'n')?,
            SpecialCode::TwoLines => write_char!(writer, 'N')?,
            SpecialCode::ShiftCursorLeft => write_char!(writer, 'l')?,
            SpecialCode::ShiftCursorRight => write_char!(writer, 'r')?,
            SpecialCode::ShiftDisplayLeft => write_char!(writer, 'L')?,
            SpecialCode::ShiftDisplayRight => write_char!(writer, 'R')?,
            SpecialCode::KillEndOfLine => write_char!(writer, 'k')?,
            SpecialCode::ReinitializeDisplay => write_char!(writer, 'I')?,
            SpecialCode::GotoXY(x, y) => write_goto_xy(writer, x, y)?,
            SpecialCode::Generator(c, x) => write_generator(writer, c, x)?,
        };

        Ok(total)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_code_eq(code: SpecialCode, expected: &str) {
        use std::io::Cursor;
        let mut buf = Cursor::new(Vec::new());
        let count = code.write_into(&mut buf).unwrap();
        let expected = expected.as_bytes();

        assert_eq!(count, expected.len());
        assert_eq!(&buf.get_ref()[0..count], expected);
    }

    #[test]
    fn simple_codes() {
        assert_code_eq(SpecialCode::DisplayOff.into(), "\x1b[Ld");
        assert_code_eq(SpecialCode::DisplayOn.into(), "\x1b[LD");
    }

    #[test]
    fn special_code_goto_xy() {
        assert_code_eq(SpecialCode::GotoXY(None, None), "\x1b[L;");
        assert_code_eq(SpecialCode::GotoXY(Some(42), None), "\x1b[Lx42;");
        assert_code_eq(SpecialCode::GotoXY(None, Some(42)), "\x1b[Ly42;");
        assert_code_eq(SpecialCode::GotoXY(Some(42), Some(32)), "\x1b[Lx42y32;");
    }

    #[test]
    fn special_code_gen() {
        assert_code_eq(
            SpecialCode::Generator(7, 0xdeadbeefdecacafe),
            "\x1b[LG7deadbeefdecacafe;",
        );
        assert_code_eq(SpecialCode::Generator(7, 0xff), "\x1b[LG700000000000000ff;");
    }
}
