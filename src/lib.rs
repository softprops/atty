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
    Stdin,
}

/// returns true if this is a tty
#[cfg(unix)]
pub fn is(stream: Stream) -> bool {
    let fd = match stream {
        Stream::Stdout => libc::STDOUT_FILENO,
        Stream::Stderr => libc::STDERR_FILENO,
        Stream::Stdin => libc::STDIN_FILENO,
    };
    unsafe { libc::isatty(fd) != 0 }
}

/// returns true if this is a tty
#[cfg(windows)]
pub fn is(stream: Stream) -> bool {
    extern crate kernel32;
    extern crate winapi;

    let handle = kernel32::GetStdHandle(match stream {
        Stream::Stdin => winapi::STD_INPUT_HANDLE,
        Stream::Stderr => winapi::STD_ERROR_HANDLE,
        Stream::Stdout => winapi::STD_OUTPUT_HANDLE,
    });
    unsafe {
        match stream {
            Stream::Stdin => {
                let mut out = 0;
                kernel32::GetConsoleMode(handle, &mut out) != 0
            }
            _ => {
                // https://msdn.microsoft.com/en-us/library/windows/desktop/ms683171(v=vs.85).aspx
                let mut buffer_info = ::std::mem::uninitialized();
                kernel32::GetConsoleScreenBufferInfo(handle, &mut buffer_info) != 0
            }
        }

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
    fn is_err() {
        assert!(is(Stream::Stderr))
    }

    #[test]
    fn is_out() {
        assert!(is(Stream::Stdout))
    }

    #[test]
    fn is_in() {
        assert!(is(Stream::Stdin))
    }
}
