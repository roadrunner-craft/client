use std::ffi::CString;

//pub fn str2cstr(s: str) -> &CString {
//    &CString::new(s).unwrap()
//}

pub fn cstr_of_size(length: usize) -> CString {
    let mut buffer: Vec<u8> = Vec::with_capacity(length + 1);
    buffer.extend([b' '].iter().cycle().take(length));
    unsafe { CString::from_vec_unchecked(buffer) }
}
