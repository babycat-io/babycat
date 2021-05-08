//! # Terminal
//!
//! The `terminal` module provides functionality to work with the terminal.
//!
//! This documentation does not contain a lot of examples. The reason is that it's fairly
//! obvious how to use this crate. Although, we do provide
//! [examples](https://github.com/crossterm-rs/crossterm/tree/master/examples) repository
//! to demonstrate the capabilities.
//!
//! Most terminal actions can be performed with commands.
//! Please have a look at [command documentation](../index.html#command-api) for a more detailed documentation.
//!
//! ## Screen Buffer
//!
//! A screen buffer is a two-dimensional array of character
//! and color data which is displayed in a terminal screen.
//!
//! The terminal has several of those buffers and is able to switch between them.
//! The default screen in which you work is called the 'main screen'.
//! The other screens are called the 'alternative screen'.
//!
//! It is important to understand that crossterm does not yet support creating screens,
//! or switch between more than two buffers, and only offers the ability to change
//! between the 'alternate' and 'main screen'.
//!
//! ### Alternate Screen
//!
//! By default, you will be working on the main screen.
//! There is also another screen called the 'alternative' screen.
//! This screen is slightly different from the main screen.
//! For example, it has the exact dimensions of the terminal window,
//! without any scroll-back area.
//!
//! Crossterm offers the possibility to switch to the 'alternative' screen,
//! make some modifications, and move back to the 'main' screen again.
//! The main screen will stay intact and will have the original data as we performed all
//! operations on the alternative screen.
//!
//! An good example of this is Vim.
//! When it is launched from bash, a whole new buffer is used to modify a file.
//! Then, when the modification is finished, it closes again and continues on the main screen.
//!
//! ### Raw Mode
//!
//! By default, the terminal functions in a certain way.
//! For example, it will move the cursor to the beginning of the next line when the input hits the end of a line.
//! Or that the backspace is interpreted for character removal.
//!
//! Sometimes these default modes are irrelevant,
//! and in this case, we can turn them off.
//! This is what happens when you enable raw modes.
//!
//! Those modes will be set when enabling raw modes:
//!
//! - Input will not be forwarded to screen
//! - Input will not be processed on enter press
//! - Input will not be line buffered (input sent byte-by-byte to input buffer)
//! - Special keys like backspace and CTL+C will not be processed by terminal driver
//! - New line character will not be processed therefore `println!` can't be used, use `write!` instead
//!
//! ## Examples
//!
//! ```no_run
//! use std::io::{stdout, Write};
//! use crossterm::{execute, Result, terminal::{ScrollUp, SetSize, size}};
//!
//! fn main() -> Result<()> {
//!     let (cols, rows) = size()?;
//!     // Resize terminal and scroll up.
//!     execute!(
//!         stdout(),
//!         SetSize(10, 10),
//!         ScrollUp(5)
//!     )?;
//!
//!     // Be a good citizen, cleanup
//!     execute!(stdout(), SetSize(cols, rows))?;
//!     Ok(())
//! }
//! ```
//!
//! For manual execution control check out [crossterm::queue](../macro.queue.html).

use std::fmt;

#[cfg(windows)]
use crossterm_winapi::{ConsoleMode, Handle, ScreenBuffer};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(windows)]
use winapi::um::wincon::ENABLE_WRAP_AT_EOL_OUTPUT;

#[doc(no_inline)]
use crate::Command;
use crate::{impl_display, Result};

mod ansi;
pub(crate) mod sys;

/// Enables raw mode.
///
/// Please have a look at the [raw mode](./#raw-mode) section.
pub fn enable_raw_mode() -> Result<()> {
    sys::enable_raw_mode()
}

/// Disables raw mode.
///
/// Please have a look at the [raw mode](./#raw-mode) section.
pub fn disable_raw_mode() -> Result<()> {
    sys::disable_raw_mode()
}

/// Returns the terminal size `(columns, rows)`.
///
/// The top left cell is represented `(1, 1)`.
pub fn size() -> Result<(u16, u16)> {
    sys::size()
}

/// Disables line wrapping.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DisableLineWrap;

impl Command for DisableLineWrap {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        f.write_str(ansi::DISABLE_LINE_WRAP_CSI_SEQUENCE)
    }

    #[cfg(windows)]
    fn execute_winapi(&self, _writer: impl FnMut() -> Result<()>) -> Result<()> {
        let screen_buffer = ScreenBuffer::current()?;
        let console_mode = ConsoleMode::from(screen_buffer.handle().clone());
        let new_mode = console_mode.mode()? & !ENABLE_WRAP_AT_EOL_OUTPUT;
        console_mode.set_mode(new_mode)?;
        Ok(())
    }
}

/// Enable line wrapping.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EnableLineWrap;

impl Command for EnableLineWrap {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        f.write_str(ansi::ENABLE_LINE_WRAP_CSI_SEQUENCE)
    }

    #[cfg(windows)]
    fn execute_winapi(&self, _writer: impl FnMut() -> Result<()>) -> Result<()> {
        let screen_buffer = ScreenBuffer::current()?;
        let console_mode = ConsoleMode::from(screen_buffer.handle().clone());
        let new_mode = console_mode.mode()? | ENABLE_WRAP_AT_EOL_OUTPUT;
        console_mode.set_mode(new_mode)?;
        Ok(())
    }
}

/// A command that switches to alternate screen.
///
/// # Notes
///
/// * Commands must be executed/queued for execution otherwise they do nothing.
/// * Use [LeaveAlternateScreen](./struct.LeaveAlternateScreen.html) command to leave the entered alternate screen.
///
/// # Examples
///
/// ```no_run
/// use std::io::{stdout, Write};
/// use crossterm::{execute, Result, terminal::{EnterAlternateScreen, LeaveAlternateScreen}};
///
/// fn main() -> Result<()> {
///     execute!(stdout(), EnterAlternateScreen)?;
///
///     // Do anything on the alternate screen
///
///     execute!(stdout(), LeaveAlternateScreen)
/// }
/// ```
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EnterAlternateScreen;

impl Command for EnterAlternateScreen {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        f.write_str(ansi::ENTER_ALTERNATE_SCREEN_CSI_SEQUENCE)
    }

    #[cfg(windows)]
    fn execute_winapi(&self, _writer: impl FnMut() -> Result<()>) -> Result<()> {
        let alternate_screen = ScreenBuffer::create();
        alternate_screen.show()?;
        Ok(())
    }
}

/// A command that switches back to the main screen.
///
/// # Notes
///
/// * Commands must be executed/queued for execution otherwise they do nothing.
/// * Use [EnterAlternateScreen](./struct.EnterAlternateScreen.html) to enter the alternate screen.
///
/// # Examples
///
/// ```no_run
/// use std::io::{stdout, Write};
/// use crossterm::{execute, Result, terminal::{EnterAlternateScreen, LeaveAlternateScreen}};
///
/// fn main() -> Result<()> {
///     execute!(stdout(), EnterAlternateScreen)?;
///
///     // Do anything on the alternate screen
///
///     execute!(stdout(), LeaveAlternateScreen)
/// }
/// ```
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LeaveAlternateScreen;

impl Command for LeaveAlternateScreen {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        f.write_str(ansi::LEAVE_ALTERNATE_SCREEN_CSI_SEQUENCE)
    }

    #[cfg(windows)]
    fn execute_winapi(&self, _writer: impl FnMut() -> Result<()>) -> Result<()> {
        let screen_buffer = ScreenBuffer::from(Handle::current_out_handle()?);
        screen_buffer.show()?;
        Ok(())
    }
}

/// Different ways to clear the terminal buffer.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum ClearType {
    /// All cells.
    All,
    /// All cells from the cursor position downwards.
    FromCursorDown,
    /// All cells from the cursor position upwards.
    FromCursorUp,
    /// All cells at the cursor row.
    CurrentLine,
    /// All cells from the cursor position until the new line.
    UntilNewLine,
}

/// A command that scrolls the terminal screen a given number of rows up.
///
/// # Notes
///
/// Commands must be executed/queued for execution otherwise they do nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScrollUp(pub u16);

impl Command for ScrollUp {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        ansi::scroll_up_csi_sequence(f, self.0)
    }

    #[cfg(windows)]
    fn execute_winapi(&self, _writer: impl FnMut() -> Result<()>) -> Result<()> {
        sys::scroll_up(self.0)
    }
}

/// A command that scrolls the terminal screen a given number of rows down.
///
/// # Notes
///
/// Commands must be executed/queued for execution otherwise they do nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScrollDown(pub u16);

impl Command for ScrollDown {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        ansi::scroll_down_csi_sequence(f, self.0)
    }

    #[cfg(windows)]
    fn execute_winapi(&self, _writer: impl FnMut() -> Result<()>) -> Result<()> {
        sys::scroll_down(self.0)
    }
}

/// A command that clears the terminal screen buffer.
///
/// See the [`ClearType`](enum.ClearType.html) enum.
///
/// # Notes
///
/// Commands must be executed/queued for execution otherwise they do nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Clear(pub ClearType);

impl Command for Clear {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        f.write_str(match self.0 {
            ClearType::All => ansi::CLEAR_ALL_CSI_SEQUENCE,
            ClearType::FromCursorDown => ansi::CLEAR_FROM_CURSOR_DOWN_CSI_SEQUENCE,
            ClearType::FromCursorUp => ansi::CLEAR_FROM_CURSOR_UP_CSI_SEQUENCE,
            ClearType::CurrentLine => ansi::CLEAR_FROM_CURRENT_LINE_CSI_SEQUENCE,
            ClearType::UntilNewLine => ansi::CLEAR_UNTIL_NEW_LINE_CSI_SEQUENCE,
        })
    }

    #[cfg(windows)]
    fn execute_winapi(&self, _writer: impl FnMut() -> Result<()>) -> Result<()> {
        sys::clear(self.0)
    }
}

/// A command that sets the terminal size `(columns, rows)`.
///
/// # Notes
///
/// Commands must be executed/queued for execution otherwise they do nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SetSize(pub u16, pub u16);

impl Command for SetSize {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        ansi::set_size_csi_sequence(f, self.0, self.1)
    }

    #[cfg(windows)]
    fn execute_winapi(&self, _writer: impl FnMut() -> Result<()>) -> Result<()> {
        sys::set_size(self.0, self.1)
    }
}

/// A command that sets the terminal title
///
/// # Notes
///
/// Commands must be executed/queued for execution otherwise they do nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SetTitle<'a>(pub &'a str);

impl<'a> Command for SetTitle<'a> {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        ansi::set_title_ansi_sequence(f, self.0)
    }

    #[cfg(windows)]
    fn execute_winapi(&self, _writer: impl FnMut() -> Result<()>) -> Result<()> {
        sys::set_window_title(self.0)
    }
}

impl_display!(for ScrollUp);
impl_display!(for ScrollDown);
impl_display!(for SetSize);
impl_display!(for Clear);

#[cfg(test)]
mod tests {
    use std::{io::stdout, thread, time};

    use crate::execute;

    use super::{size, SetSize};

    // Test is disabled, because it's failing on Travis CI
    #[test]
    #[ignore]
    fn test_resize_ansi() {
        let (width, height) = size().unwrap();

        execute!(stdout(), SetSize(35, 35)).unwrap();

        // see issue: https://github.com/eminence/terminal-size/issues/11
        thread::sleep(time::Duration::from_millis(30));

        assert_eq!((35, 35), size().unwrap());

        // reset to previous size
        execute!(stdout(), SetSize(width, height)).unwrap();

        // see issue: https://github.com/eminence/terminal-size/issues/11
        thread::sleep(time::Duration::from_millis(30));

        assert_eq!((width, height), size().unwrap());
    }
}
