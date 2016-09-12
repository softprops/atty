//! atty is a simple utility that answers one question
//! > is this a tty?
//!
//! usage is just as simple
//!
//! ```
//! if atty::is(atty::Stream::Stdout) {
//!   println!("i'm a tty")
//! }
//! ```
//!
//! ```
//! if atty::isnt(atty::Stream::Stdout) {
//!   println!("i'm not a tty")
//! }
//! ```

extern crate libc;

/// possible stream sources
pub enum Stream {
    Stdout,
    Stderr,
    Stdin
}

/// returns true if this is a tty
#[cfg(unix)]
pub fn is(stream: Stream) -> bool {
    let fd = match stream {
        Stream::Stdout => libc::STDOUT_FILENO,
        Stream::Stderr => libc::STDERR_FILENO,
        Stream::Stdin => libc::STDIN_FILENO
    };
    unsafe { libc::isatty(fd) != 0 }
}

/// returns true if this is a tty
#[cfg(windows)]
pub fn is(stream: Stream) -> bool {
    extern crate kernel32;
    extern crate winapi;

    let handle = match stream {
        Stream::Stdout => winapi::STD_OUTPUT_HANDLE,
        Stream::Stderr => winapi::STD_ERROR_HANDLE,
        Stream::Stdin => winapi::STD_INPUT_HANDLE
    };

    unsafe {
        let handle = kernel32::GetStdHandle(handle);
        let mut out = 0;
        panic!("is invalid? {:#?}  result {:#?} last err {:#?}",
        winapi::INVALID_HANDLE_VALUE == handle,
        kernel32::GetConsoleMode(handle, &mut out),
        kernel32::GetLastError());
        kernel32::GetConsoleMode(handle, &mut out) != 0
    }
}

/// returns true if this is _not_ a tty
pub fn isnt(stream: Stream) -> bool {
    !is(stream)
}

#[cfg(test)]
mod tests {
    use super::{is, Stream};

    #[test]
    fn is_out() {
        assert!(is(Stream::Stdout))
    }

    #[test]
    fn is_err() {
        assert!(is(Stream::Stderr))
    }

    #[test]
    fn is_in() {
        assert!(is(Stream::Stdin))
    }
}
