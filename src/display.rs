use super::*;

use std::fmt::{self, Display, Formatter, Write};

impl<'a> Display for Piece<'a> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            String(s) => {
                if let Some(c @ ('{' | '}')) = s.chars().next() {
                    f.write_char(c)?;
                }
                s.fmt(f)
            }
            NextArgument(a) => a.fmt(f),
        }
    }
}

impl<'a> Display for Argument<'a> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_char('{')?;
        self.position.fmt(f)?;
        self.format.fmt(f)?;
        f.write_char('}')
    }
}

impl<'a> Display for FormatSpec<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.is_empty() {
            return Ok(());
        }
        f.write_char(':')?;
        if let Some(fill) = self.fill {
            f.write_char(fill)?;
        }
        self.align.fmt(f)?;
        if self.flags & (1 << FlagSignPlus as u32) != 0 {
            f.write_char('+')?;
        } else if self.flags & (1 << FlagSignMinus as u32) != 0 {
            f.write_char('-')?;
        }
        if self.flags & (1 << FlagAlternate as u32) != 0 {
            f.write_char('#')?;
        }
        if self.flags & (1 << FlagSignAwareZeroPad as u32) != 0 {
            f.write_char('0')?;
        }
        self.width.fmt(f)?;
        if self.precision != CountImplied {
            f.write_char('.')?;
            self.precision.fmt(f)?;
        }
        if self.flags & (1 << FlagDebugLowerHex as u32) != 0 {
            f.write_char('x')?;
        } else if self.flags & (1 << FlagDebugUpperHex as u32) != 0 {
            f.write_char('X')?;
        }
        self.ty.fmt(f)
    }
}

impl<'a> Display for Position<'a> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ArgumentImplicitlyIs(_) => Ok(()),
            ArgumentIs(p) => p.fmt(f),
            ArgumentNamed(s) => s.fmt(f),
        }
    }
}

impl Display for Alignment {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            AlignLeft => f.write_char('<'),
            AlignRight => f.write_char('>'),
            AlignCenter => f.write_char('^'),
            AlignUnknown => Ok(()),
        }
    }
}

impl<'a> Display for Count<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            CountIs(p) => p.fmt(f),
            CountIsName(s, _) => {
                s.fmt(f)?;
                f.write_char('$')
            }
            CountIsParam(p) => {
                p.fmt(f)?;
                f.write_char('$')
            }
            CountIsStar(_) => f.write_char('*'),
            CountImplied => Ok(()),
        }
    }
}

impl Display for ParseError {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.span.start, self.description)
    }
}

impl std::error::Error for ParseError {}
