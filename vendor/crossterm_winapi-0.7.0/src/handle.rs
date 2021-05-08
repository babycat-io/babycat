//! This module contains some logic for working with the console handle.

use std::io::{self, Result};
use std::ops::Deref;
use std::ptr::null_mut;
use std::sync::Arc;

use winapi::shared::minwindef::DWORD;
use winapi::um::{
    fileapi::{CreateFileW, OPEN_EXISTING},
    handleapi::{CloseHandle, INVALID_HANDLE_VALUE},
    processenv::GetStdHandle,
    winbase::{STD_INPUT_HANDLE, STD_OUTPUT_HANDLE},
    winnt::{FILE_SHARE_READ, FILE_SHARE_WRITE, GENERIC_READ, GENERIC_WRITE, HANDLE},
};

/// This enum represents the different handles that could be requested.
///
/// Some more details could be found [here](https://docs.microsoft.com/en-us/windows/console/getstdhandle#parameters)
#[derive(Debug, Clone, Copy)]
pub enum HandleType {
    /// This represents the `STD_OUTPUT_HANDLE`
    OutputHandle,
    /// This represents the `STD_INPUT_HANDLE`
    InputHandle,
    /// This represents the `CONOUT$` file handle
    /// When using multiple screen buffers this will always point to the to the current screen output buffer.
    CurrentOutputHandle,
    /// This represents the `CONIN$` file handle.
    /// When using multiple screen buffers this will always point to the to the current screen input buffer.
    CurrentInputHandle,
}

/// Inner structure for closing a handle on Drop.
///
/// The second parameter indicates if the HANDLE is exclusively owned or not.
/// A non-exclusive handle can be created using for example
/// `Handle::input_handle` or `Handle::output_handle`, which corresponds to
/// stdin and stdout respectively.
#[derive(Debug)]
struct Inner {
    handle: HANDLE,
    is_exclusive: bool,
}

impl Inner {
    fn new_exclusive(handle: HANDLE) -> Self {
        Inner {
            handle,
            is_exclusive: true,
        }
    }

    fn new_shared(handle: HANDLE) -> Self {
        Inner {
            handle,
            is_exclusive: false,
        }
    }
}

impl Drop for Inner {
    fn drop(&mut self) {
        if self.is_exclusive {
            assert!(
                unsafe { CloseHandle(self.handle) != 0 },
                "failed to close handle"
            )
        }
    }
}

unsafe impl Send for Inner {}

unsafe impl Sync for Inner {}

/// This abstracts away some WinaApi calls to set and get some console handles.
///
// Wraps the underlying WinApi type: [HANDLE]
#[derive(Debug, Clone)]
pub struct Handle {
    handle: Arc<Inner>,
}

impl Handle {
    pub fn new(handle: HandleType) -> Result<Handle> {
        match handle {
            HandleType::OutputHandle => Handle::output_handle(),
            HandleType::InputHandle => Handle::input_handle(),
            HandleType::CurrentOutputHandle => Handle::current_out_handle(),
            HandleType::CurrentInputHandle => Handle::current_in_handle(),
        }
    }

    /// Construct a handle from a raw handle.
    ///
    /// # Safety
    ///
    /// This is unsafe since there is not guarantee that the underlying HANDLE is thread-safe to implement `Send` and `Sync`.
    /// Most HANDLE's however, are thread safe.
    pub unsafe fn from_raw(handle: HANDLE) -> Self {
        Self {
            handle: Arc::new(Inner::new_exclusive(handle)),
        }
    }

    /// Get the handle of the active screen buffer.
    /// When using multiple screen buffers this will always point to the to the current screen output buffer.
    ///
    /// On success this function returns the `HANDLE` to `STD_OUTPUT_HANDLE`.
    ///
    /// This function uses `CONOUT$` to create a file handle to the current output buffer.
    ///
    /// Wraps the underlying function call: [CreateFileW]
    /// link: [https://docs.microsoft.com/en-us/windows/desktop/api/fileapi/nf-fileapi-createfilew]
    pub fn current_out_handle() -> Result<Handle> {
        let utf16: Vec<u16> = "CONOUT$\0".encode_utf16().collect();
        let utf16_ptr: *const u16 = utf16.as_ptr();

        let handle = unsafe {
            CreateFileW(
                utf16_ptr,
                GENERIC_READ | GENERIC_WRITE,
                FILE_SHARE_READ | FILE_SHARE_WRITE,
                null_mut(),
                OPEN_EXISTING,
                0,
                null_mut(),
            )
        };

        if !Self::is_valid_handle(&handle) {
            return Err(io::Error::last_os_error());
        }

        Ok(Handle {
            handle: Arc::new(Inner::new_exclusive(handle)),
        })
    }

    /// Get the handle of the active input screen buffer.
    /// When using multiple screen buffers this will always point to the to the current screen input buffer.
    ///
    /// On success this function returns the `HANDLE` to `STD_INPUT_HANDLE`.
    ///
    /// This function uses `CONIN$` to create a file handle to the current input buffer.
    ///
    /// Wraps the underlying function call: [CreateFileW]
    /// link: [https://docs.microsoft.com/en-us/windows/desktop/api/fileapi/nf-fileapi-createfilew]
    pub fn current_in_handle() -> Result<Handle> {
        let utf16: Vec<u16> = "CONIN$\0".encode_utf16().collect();
        let utf16_ptr: *const u16 = utf16.as_ptr();

        let handle = unsafe {
            CreateFileW(
                utf16_ptr,
                GENERIC_READ | GENERIC_WRITE,
                FILE_SHARE_READ | FILE_SHARE_WRITE,
                null_mut(),
                OPEN_EXISTING,
                0,
                null_mut(),
            )
        };

        if !Handle::is_valid_handle(&handle) {
            return Err(io::Error::last_os_error());
        }

        Ok(Handle {
            handle: Arc::new(Inner::new_exclusive(handle)),
        })
    }

    /// Get the handle of the output screen buffer.
    ///
    /// On success this function returns the `HANDLE` to `STD_OUTPUT_HANDLE`.
    ///
    /// Wraps the underlying function call: [GetStdHandle] whit argument `STD_OUTPUT_HANDLE`
    /// link: [https://docs.microsoft.com/en-us/windows/console/getstdhandle]
    pub fn output_handle() -> Result<Handle> {
        Self::std_handle(STD_OUTPUT_HANDLE)
    }

    /// Get the handle of the input screen buffer.
    ///
    /// On success this function returns the `HANDLE` to `STD_INPUT_HANDLE`.
    ///
    /// Wraps the underlying function call: [GetStdHandle] whit argument `STD_INPUT_HANDLE`
    /// link: [https://docs.microsoft.com/en-us/windows/console/getstdhandle]
    pub fn input_handle() -> Result<Handle> {
        Self::std_handle(STD_INPUT_HANDLE)
    }

    fn std_handle(which_std: DWORD) -> Result<Handle> {
        let handle = unsafe { GetStdHandle(which_std) };

        if !Handle::is_valid_handle(&handle) {
            Err(io::Error::last_os_error())
        } else {
            Ok(Handle {
                handle: Arc::new(Inner::new_shared(handle)),
            })
        }
    }

    /// Checks if the console handle is an invalid handle value.
    ///
    /// This is done by checking if the passed `HANDLE` is equal to `INVALID_HANDLE_VALUE`
    pub fn is_valid_handle(handle: &HANDLE) -> bool {
        if *handle == INVALID_HANDLE_VALUE {
            false
        } else {
            true
        }
    }
}

impl Deref for Handle {
    type Target = HANDLE;

    fn deref(&self) -> &HANDLE {
        &self.handle.handle
    }
}

#[cfg(test)]
mod tests {
    use super::{Handle, HandleType};

    #[test]
    fn test_get_handle() {
        assert!(Handle::new(HandleType::OutputHandle).is_ok());
        assert!(Handle::new(HandleType::InputHandle).is_ok());
        assert!(Handle::new(HandleType::CurrentOutputHandle).is_ok());
        assert!(Handle::new(HandleType::CurrentInputHandle).is_ok());
    }
}
