use std::fmt;
use std::mem::zeroed;

use winapi::um::wincon::CONSOLE_SCREEN_BUFFER_INFO;

use super::{Coord, Size, WindowPositions};

/// This type is a wrapper for `CONSOLE_SCREEN_BUFFER_INFO` and has some methods to extract information from it.
///
/// Wraps the underlying type: [CONSOLE_SCREEN_BUFFER_INFO]
/// link: [https://docs.microsoft.com/en-us/windows/console/console-screen-buffer-info-str]
// TODO: replace the innards of this type with our own, more friendly types, like Coord.
// This will obviously be a breaking change.
#[derive(Clone)]
pub struct ScreenBufferInfo(pub CONSOLE_SCREEN_BUFFER_INFO);

// TODO: replace this with a derive ASAP
impl fmt::Debug for ScreenBufferInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("ScreenBufferInfo")
            .field("dwSize", &self.buffer_size())
            .field("dwCursorPosition", &self.cursor_pos())
            .field("wAttributes", &self.attributes()) // TODO: hex print this
            .field("srWindow", &self.terminal_window())
            .field(
                "dwMaximumWindowSize",
                &Size::from(self.0.dwMaximumWindowSize),
            )
            .finish()
    }
}

impl ScreenBufferInfo {
    pub fn new() -> ScreenBufferInfo {
        ScreenBufferInfo(unsafe { zeroed() })
    }

    /// This will return the buffer size.
    ///
    /// Will take `dwSize` from the current screen buffer and convert it into the `Size`.
    pub fn buffer_size(&self) -> Size {
        Size::from(self.0.dwSize)
    }

    /// This will return the terminal size.
    ///
    /// Will calculate the width and height from `srWindow` and convert it into a `Size`.
    pub fn terminal_size(&self) -> Size {
        (Size::new(
            self.0.srWindow.Right - self.0.srWindow.Left,
            self.0.srWindow.Bottom - self.0.srWindow.Top,
        ))
    }

    /// This will return the terminal window properties.
    ///
    /// Will take `srWindow` and convert it into the `WindowPositions` type.
    pub fn terminal_window(&self) -> WindowPositions {
        WindowPositions::from(self.0)
    }

    /// This will return the terminal window properties.
    ///
    /// Will take `wAttributes` from the current screen buffer.
    pub fn attributes(&self) -> u16 {
        self.0.wAttributes
    }

    /// This will return the current cursor position.
    ///
    /// Will take `dwCursorPosition` from the current screen buffer.
    pub fn cursor_pos(&self) -> Coord {
        Coord::from(self.0.dwCursorPosition)
    }
}

impl From<CONSOLE_SCREEN_BUFFER_INFO> for ScreenBufferInfo {
    fn from(csbi: CONSOLE_SCREEN_BUFFER_INFO) -> Self {
        ScreenBufferInfo(csbi)
    }
}
