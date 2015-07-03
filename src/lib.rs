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
pub fn is() -> bool {
  let r = unsafe { libc::isatty(libc::STDOUT_FILENO) };
  r != 0
}

/// returns true if this is _not_ a tty
pub fn isnt() -> bool {
  !is()
}

#[test]
fn it_works() {
}
