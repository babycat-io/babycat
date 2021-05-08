use std::io::{Error, Result};

use winapi::um::consoleapi::{GetConsoleMode, SetConsoleMode};

use super::{is_true, Handle, HandleType};

/// This abstracts away some WinaApi calls to set and get the console mode.
///
/// Wraps the underlying function call: [SetConsoleMode]
/// link: [https://docs.microsoft.com/en-us/windows/console/setconsolemode]
///
/// Wraps the underlying function call: [GetConsoleMode]
/// link: [https://docs.microsoft.com/en-us/windows/console/getconsolemode]
#[derive(Debug, Clone)]
pub struct ConsoleMode {
    // the handle used for the functions of this type.
    handle: Handle,
}

impl ConsoleMode {
    /// Create a new `ConsoleMode` instance.
    ///
    /// This will use the `STD_OUTPUT_HANDLE` as default handle.
    /// When you explicitly want to specify the handle used for the function calls use `ConsoleMode::from(handle)` instead.
    pub fn new() -> Result<ConsoleMode> {
        Ok(ConsoleMode {
            handle: Handle::new(HandleType::OutputHandle)?,
        })
    }

    /// Set the console mode to the given console mode.
    ///
    /// This function sets the `dwMode`.
    ///
    /// Wraps the underlying function call: [SetConsoleMode]
    /// link: [https://docs.microsoft.com/en-us/windows/console/setconsolemode]
    pub fn set_mode(&self, console_mode: u32) -> Result<()> {
        unsafe {
            if !is_true(SetConsoleMode(*self.handle, console_mode)) {
                return Err(Error::last_os_error());
            }
        }
        Ok(())
    }

    /// Get the console mode.
    ///
    /// This function returns the `lpMode`.
    ///
    /// Wraps the underlying function call: [GetConsoleMode]
    /// link: [https://docs.microsoft.com/en-us/windows/console/getconsolemode]
    pub fn mode(&self) -> Result<u32> {
        let mut console_mode = 0;
        unsafe {
            if !is_true(GetConsoleMode(*self.handle, &mut console_mode)) {
                return Err(Error::last_os_error());
            }
        }
        Ok(console_mode)
    }
}

impl From<Handle> for ConsoleMode {
    fn from(handle: Handle) -> Self {
        ConsoleMode { handle }
    }
}

#[cfg(test)]
mod tests {
    use super::ConsoleMode;

    // TODO - Test is ignored, because it's failing on Travis CI
    #[test]
    #[ignore]
    fn test_set_get_mode() {
        let mode = ConsoleMode::new().unwrap();

        let original_mode = mode.mode().unwrap();

        mode.set_mode(0x0004).unwrap();
        let console_mode = mode.mode().unwrap();
        assert_eq!(console_mode & 0x0004, mode.mode().unwrap());

        mode.set_mode(original_mode).unwrap();
    }
}
