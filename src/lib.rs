extern crate cpython;

use cpython::{PyResult, Python, py_module_initializer, py_fn};
use std::char;


py_module_initializer!(polyline_encoding, |py, m| {
    m.add(py, "__doc__", "This module is implemented in Rust.")?;
    m.add(py, "encode_unsigned_number", py_fn!(py, encode_unsigned_number(num: i64)))?;
    m.add(py, "encode_signed_number", py_fn!(py, encode_signed_number(num: i64)))?;
    Ok(())
});


fn encode_unsigned_number_rust(num: i64) -> String  {
    let mut encoded = String::from("");
    let mut tmp: i64 = num;
    while tmp >= 0x20 {
        encoded.push_str(&(((0x20 | (tmp as u8 & 0x1f)) + 63) as char).to_string());
        tmp >>= 5;
    }
    encoded.push_str(&((tmp as u8 + 63) as char).to_string());
    return encoded;
}

fn encode_unsigned_number(_py: Python, num: i64) -> PyResult<String>  {
    let encoded: String = encode_unsigned_number_rust(num);
    Ok(encoded)
}

fn encode_signed_number(_py: Python, num: i64) -> PyResult<String> {
    let mut sgn_num: i64 = num << 1;
    if num < 0 {
        sgn_num = !sgn_num;
    }
    Ok(encode_unsigned_number_rust(sgn_num))
}
