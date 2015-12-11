//! atty is a simple utility that answers one question
//! > is this a tty?
//!
//! usage is just as simple
//!
//! ```
//! if atty::is() {
//!   println!("i'm a tty")
//! }
//! ```
//!
//! ```
//! if atty::isnt() {
//!   println!("i'm not a tty")
//! }
//! ```

extern crate libc;

/// returns true if this is a tty
#[cfg(unix)]
pub fn is() -> bool {
  let r = unsafe { libc::isatty(libc::STDOUT_FILENO) };
  r != 0
}

/// returns true if this is a tty
#[cfg(windows)]
pub fn is() -> bool {
    extern crate kernel32;
    extern crate winapi;
    unsafe {
        let handle = kernel32::GetStdHandle(if fd == libc::STDOUT_FILENO {
            winapi::winbase::STD_OUTPUT_HANDLE
        } else {
            winapi::winbase::STD_ERROR_HANDLE
        });
        let mut out = 0;
        kernel32::GetConsoleMode(handle, &mut out) != 0
    }
}

/// returns true if this is _not_ a tty
pub fn isnt() -> bool {
  !is()
}

#[cfg(test)]
mod tests {
    use super::is;

    #[test]
    fn is_test() {
        assert!(is())
    }
}
