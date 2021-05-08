use crate::Handle;
use std::{io, ptr};
use winapi::um::synchapi::{CreateSemaphoreW, ReleaseSemaphore};

#[derive(Clone, Debug)]
pub struct Semaphore(Handle);

impl Semaphore {
    /// Construct a new semaphore.
    pub fn new() -> io::Result<Self> {
        let handle = unsafe { CreateSemaphoreW(ptr::null_mut(), 0, 1, ptr::null_mut()) };

        if handle == ptr::null_mut() {
            return Err(io::Error::last_os_error());
        }

        let handle = unsafe { Handle::from_raw(handle) };
        Ok(Self(handle))
    }

    /// Release a permit on the semaphore.
    pub fn release(&self) -> io::Result<()> {
        let result = unsafe { ReleaseSemaphore(*self.0, 1, ptr::null_mut()) };

        if result == 0 {
            return Err(io::Error::last_os_error());
        }

        Ok(())
    }

    /// Access the underlying handle to the semaphore.
    pub fn handle(&self) -> &Handle {
        &self.0
    }
}

unsafe impl Send for Semaphore {}

unsafe impl Sync for Semaphore {}
