use std::{ffi::CString, str::FromStr};

use bigdecimal::{BigDecimal, FromPrimitive};
use num_bigint::BigUint;

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

pub fn hex_string_to_u8_array(hex_str: &str, pad_left: bool) -> Result<Vec<u8>, Error> {
    let pad = "0".repeat(hex_str.len() % 8);
    let lower = hex_str.to_lowercase();
    let hex_str = if pad_left {
        format!("{pad}{lower}")
    } else {
        format!("{lower}{pad}")
    };

    println!("hex str: {}", hex_str);

    for char in hex_str.chars() {
        if !char.is_ascii_hexdigit() {
            return Err(Error::InvalidHexString);
        };
    }

    let mut out = Vec::new();

    for chunk in hex_str.as_bytes().chunks(2) {
        let mut chunk_value = 0u8;
        for (i, byte) in chunk.iter().enumerate() {
            let byte = match byte {
                0x61..=0x66 => byte - 0x61 + 10, // a-f
                0x30..=0x39 => byte - 0x30,      // 0-9
                _ => 0,                          // Make the compiler happy
            };
            chunk_value |= byte << (4 * (i & 1));
        }
        out.push(chunk_value);
    }

    Ok(out)
}

pub fn fractional_value_string_from_u8_array(arr: &[u8]) -> String {
    let mut out = BigDecimal::from(0);

    for (i, byte) in arr.iter().enumerate() {
        let mut byte = *byte;
        for b in 0..7 {
            println!("{:08b}", byte);
            if (byte & 0x80) != 0 {
                println!("Adding: {}", b);
                println!(
                    "resolved: {}",
                    BigDecimal::from(2).powi(-((i as i64 * 8) + b as i64))
                );
                out += BigDecimal::from(2).powi(-((i as i64 * 8) + b as i64));
            }
            byte <<= 1;
        }
    }

    out.to_plain_string()
}

pub fn hex_float_strings_to_string(
    whole_part: &str,
    fractional_part: &str,
    exponent: &str,
) -> String {
    //let fractional_part =
    "".into()
}
