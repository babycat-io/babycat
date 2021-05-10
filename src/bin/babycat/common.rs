use log::error;
pub trait UnwrapOrExit<T, E> {
    fn unwrap_or_exit(self) -> T;
}

impl<T, E: std::fmt::Display> UnwrapOrExit<T, E> for std::result::Result<T, E> {
    fn unwrap_or_exit(self) -> T {
        match self {
            Ok(t) => t,
            Err(e) => exit_with_msg(&e.to_string()),
        }
    }
}

pub fn exit_with_msg(msg: &str) -> ! {
    error!("{}", msg);
    std::process::exit(1);
}
