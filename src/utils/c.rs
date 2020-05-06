use std::ffi::CString;

#[macro_export]
macro_rules! cstr {
    ($literal:expr) => {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(concat!($literal, "\0").as_bytes())
    };
}

pub fn str2cstr(s: &str) -> CString {
    CString::new(s).unwrap()
}

pub fn cstr_with_size(length: usize) -> CString {
    let mut buffer: Vec<u8> = Vec::with_capacity(length + 1);
    buffer.extend([b' '].iter().cycle().take(length));
    unsafe { CString::from_vec_unchecked(buffer) }
}
