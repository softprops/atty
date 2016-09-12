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

    let handle = match stream {
        Stream::Stdout => winapi::STD_OUTPUT_HANDLE,
        Stream::Stderr => winapi::STD_ERROR_HANDLE,
        Stream::Stdin => winapi::STD_INPUT_HANDLE,
    };

    unsafe {
        match stream {
            Stream::Stdin => {
                let mut out = 0;
                kernel32::GetConsoleMode(kernel32::GetStdHandle(winapi::STD_INPUT_HANDLE),
                                         &mut out) != 0
            }
            out => {
                // note: there is no CONERR, only CONOUT
                let handle = kernel32::CreateFileA(b"CONOUT$\0".as_ptr() as *const i8,
                                                   winapi::GENERIC_READ | winapi::GENERIC_WRITE,
                                                   winapi::FILE_SHARE_WRITE,
                                                   ::std::ptr::null_mut(),
                                                   winapi::OPEN_EXISTING,
                                                   0,
                                                   ::std::ptr::null_mut());
                // https://msdn.microsoft.com/en-us/library/windows/desktop/ms683171(v=vs.85).aspx
                let mut buffer_info = ::std::mem::uninitialized();
                let ret = kernel32::GetConsoleScreenBufferInfo(handle, &mut buffer_info);
                let last_err = kernel32::GetLastError();
                panic!("is invalid? {:#?}  result {:#?} last err {:#?}",
                       winapi::INVALID_HANDLE_VALUE == handle,
                       ret,
                       last_err);
                ret != 0
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
