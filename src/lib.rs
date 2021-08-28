extern crate cpython;

use cpython::{Python, PyErr, PyResult, PyDict , PyList, py_module_initializer, py_fn, exc};
use std::char;


py_module_initializer!(gps_encoding, |py, m| {
    m.add(py, "__doc__", "This module is implemented in Rust.")?;
    m.add(py, "encode_unsigned_number", py_fn!(py, encode_unsigned_number(num: i64)))?;
    m.add(py, "encode_signed_number", py_fn!(py, encode_signed_number(num: i64)))?;
    m.add(py, "encode_data", py_fn!(py, encode_data(data: &PyList)))?;
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

fn encode_signed_number_rust(num: i64) -> String {
    let mut sgn_num: i64 = num << 1;
    if num < 0 {
        sgn_num = !sgn_num;
    }
    return encode_unsigned_number_rust(sgn_num);
}

fn encode_unsigned_number(_py: Python, num: i64) -> PyResult<String>  {
    let encoded: String = encode_unsigned_number_rust(num);
    Ok(encoded)
}

fn encode_signed_number(_py: Python, num: i64) -> PyResult<String>  {
    let encoded: String = encode_signed_number_rust(num);
    Ok(encoded)
}

fn encode_data(_py: Python, data: &PyList) -> PyResult<String> {
    const YEAR2010: i64 = 1262304000;
    let mut prev_t: i64 = YEAR2010;
    let mut prev_lat: f64 = 0.0;
    let mut prev_lon: f64 = 0.0;

    let mut result: String = "".to_owned();
    
    let mut is_first: bool = true;
    
    for py_pt in data.iter(_py) {
        let pt: PyDict = py_pt.extract(_py)?;
        if !pt.contains(_py, "timestamp").unwrap() || !pt.contains(_py, "latitude").unwrap() || !pt.contains(_py, "longitude").unwrap() {
            return Err(PyErr::new::<exc::ValueError, _>(_py, "invalid list, item does not contains a valid GPS data dict"));
        }

        let tim: i64 = pt.get_item(_py, "timestamp").unwrap().extract(_py)?;
        let tim_d: i64 = tim - prev_t;
        if is_first {
            result.push_str(&encode_signed_number_rust(tim_d));
            is_first = false;
        } else if tim_d < 0 {
            return Err(PyErr::new::<exc::ValueError, _>(_py, "invalid timestamp, list should be sorted by increasing timestamp"));
        } else {
            result.push_str(&encode_unsigned_number_rust(tim_d));
        }

        let lat: f64 = pt.get_item(_py, "latitude").unwrap().extract(_py)?;
        let lat_d: i64 = ((lat - prev_lat) * 1e5).round() as i64;
        result.push_str(&encode_signed_number_rust(lat_d));

        let lon: f64 = pt.get_item(_py, "longitude").unwrap().extract(_py)?;
        let lon_d: i64 = ((lon - prev_lon) * 1e5).round() as i64;
        result.push_str(&encode_signed_number_rust(lon_d));

        prev_t += tim_d;
        prev_lat += (lat_d as f64) / 1e5;
        prev_lon += (lon_d as f64) / 1e5;
    }
    Ok(result)
}
