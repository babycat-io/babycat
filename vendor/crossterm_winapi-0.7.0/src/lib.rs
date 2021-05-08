#![cfg(windows)]
#![deny(unused_imports)]

pub use self::{
    console::Console,
    console_mode::ConsoleMode,
    csbi::ScreenBufferInfo,
    handle::{Handle, HandleType},
    screen_buffer::ScreenBuffer,
    semaphore::Semaphore,
    structs::{
        ButtonState, ControlKeyState, Coord, EventFlags, InputRecord, KeyEventRecord, MouseEvent,
        Size, WindowPositions,
    },
};

mod console;
mod console_mode;
mod csbi;
mod handle;
mod screen_buffer;
mod semaphore;
mod structs;

/// Parses the given integer to an bool by checking if the value is 0 or 1.
/// This is currently used for checking if a WinApi called succeeded, this might be moved into a macro at some time.
/// So please don't use this :(.
#[inline(always)]
pub fn is_true(value: i32) -> bool {
    value != 0
}
