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

/// possible stream sources
pub enum Stream {
    Stdout,
    Stderr,
    Stdin,
}

/// returns true if this is a tty
#[cfg(unix)]
pub fn is(stream: Stream) -> bool {
    extern crate libc;

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

    unsafe {
        let handle = kernel32::GetStdHandle(match stream {
            Stream::Stdin => winapi::STD_INPUT_HANDLE,
            Stream::Stderr => winapi::STD_ERROR_HANDLE,
            Stream::Stdout => winapi::STD_OUTPUT_HANDLE,
        });
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
    #[cfg(windows)]
    fn is_err() {
        // appveyor pipes its output
        assert!(!is(Stream::Stderr))
    }

    #[test]
    #[cfg(windows)]
    fn is_out() {
        // appveyor pipes its output
        assert!(!is(Stream::Stdout))
    }

    #[test]
    #[cfg(windows)]
    fn is_in() {
        assert!(is(Stream::Stdin))
    }

    #[test]
    #[cfg(unix)]
    fn is_err() {
        assert!(is(Stream::Stderr))
    }

    #[test]
    #[cfg(unix)]
    fn is_out() {
        assert!(is(Stream::Stdout))
    }

    #[test]
    #[cfg(target_os = "macos")]
    fn is_in() {
        // macos on travis seems to pipe its input
        assert!(!is(Stream::Stdin))
    }

    #[test]
    #[cfg(all(not(target_os = "macos"), unix))]
    fn is_in() {
        assert!(is(Stream::Stdin))
    }
}
