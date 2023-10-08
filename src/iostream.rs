use core::{array::from_ref, fmt::{self, Write}, iter::once, marker::{PhantomData, PhantomPinned}};
extern crate alloc;
use alloc::vec::Vec;

#[cfg(windows)]
type WChar = u16;
#[cfg(not(windows))]
type WChar = u32;

#[repr(C)]
struct Ostream([WChar;0], PhantomData<(*mut WChar, PhantomPinned)>);

#[allow(dead_code)]
extern "C" {
    static mut at_cout: &'static mut Ostream;
    static mut at_cerr: &'static mut Ostream;
    fn push_wostream(target: &mut Ostream, text: *const [WChar;1]) -> bool;
}

impl fmt::Write for Ostream {
    fn write_str(&mut self, text: &str) -> fmt::Result {
        #[cfg(windows)]
        let text: Vec<_> = text.encode_utf16().chain(once(0u16)).collect();
        #[cfg(not(windows))]
        let text: Vec<u32> = text.chars().chain(once('\0')).map(char::into).collect();
        unsafe { push_wostream(self, from_ref(&text[0]) as *const _).then_some(()).ok_or(fmt::Error) }
    }
}

fn write(target: &mut Ostream, args: fmt::Arguments<'_>, after: Option<char>) -> fmt::Result {
    {let r = target.write_fmt(args); (r, after.and_then(|ch| target.write_char(ch).ok()))}.0
}

pub fn _ewrite(args: fmt::Arguments<'_>, after: Option<char>) -> fmt::Result {
    unsafe { self::write(at_cerr, args, after) }
}
pub fn _cwrite(args: fmt::Arguments<'_>, after: Option<char>) -> fmt::Result {
    unsafe { write(at_cout, args, after) }
}


#[macro_export]
macro_rules! eprint {
    ($($arg:tt)*) => {{
        let _ = $crate::iostream::_ewrite(format_args!($($arg)*), None);
    }};
}

#[macro_export]
macro_rules! eprintln {
    () => {
        $crate::iostream::eprint!("\n");
    };
    ($($arg:tt)*) => {{
        let _ = $crate::iostream::_ewrite(format_args!($($arg)*), '\n');
    }};
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        let _ = $crate::iostream::_cwrite(format_args!($($arg)*), None);
    }};
}

#[macro_export]
macro_rules! println {
    () => {
        $crate::iostream::print!("\n");
    };
    ($($arg:tt)*) => {{
        let _ = $crate::iostream::_cwrite(format_args!($($arg)*), Some('\n'));
    }};
}
