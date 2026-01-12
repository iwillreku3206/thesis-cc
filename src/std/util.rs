use std::ffi::CString;

use crate::error::Error;

pub fn get_string_from_memory(
    memory: &Vec<u8>,
    start_ptr: usize,
    end_ptr: usize,
) -> Result<String, Error> {
    Ok(get_string_from_memory_cstr(memory, start_ptr, end_ptr)?
        .to_string_lossy()
        .to_string())
}

pub fn get_string_from_memory_cstr(
    memory: &Vec<u8>,
    start_ptr: usize,
    end_ptr: usize,
) -> Result<CString, Error> {
    let mut end_ptr_evaluated = 0;

    loop {
        match memory.get(end_ptr_evaluated) {
            Some(0) => break,
            Some(_) => end_ptr_evaluated += 1,
            _ if end_ptr_evaluated == end_ptr && end_ptr != 0 => break,
            None => {
                return Err(Error::MemoryOutOfRange);
            }
        };
    }

    let c_str =
        unsafe { CString::from_vec_unchecked(memory[start_ptr..end_ptr_evaluated].to_vec()) };

    return Ok(c_str);
}
