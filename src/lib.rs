extern crate cpython;

use cpython::{Python, PyErr, PyResult, PyDict, PythonObject, PyList, py_module_initializer, py_fn, exc};
use std::char;


py_module_initializer!(gps_encoding, |py, m| {
    m.add(py, "__doc__", "Encode/Decode GPS data")?;
    m.add(py, "encode_data", py_fn!(py, encode_data(data: &PyList)))?;
    m.add(py, "decode_data", py_fn!(py, decode_data(encoded: String)))?;
    Ok(())
});

struct DecodingResult {
    result: i64,
    consumed: u32,
}

fn decode_unsigned_value_from_string(encoded: &String, offset: u32) -> DecodingResult {
    let enc_len: u32 = encoded.len() as u32;
    let mut i: u32 =0;
    let mut s: u32 = 0;
    let mut result: i64 = 0;
    let mut b: u8 = 0x20;
    let ptr: *const u8 = encoded.as_ptr();
    while b >= 0x20 && i + offset < enc_len {
        assert!(i + offset < enc_len);
        unsafe {
            b = *ptr.offset((i + offset) as isize) as u8 - 63;
        }
        i += 1;
        result |=  ((b & 0x1f) as i64) << s;
        s += 5;
    }
    return DecodingResult{result: result, consumed: i};
}

fn decode_signed_value_from_string(encoded: &String, offset: u32) -> DecodingResult {
    let r: DecodingResult = decode_unsigned_value_from_string(&encoded, offset);
    let result: i64 = r.result;
    if result & 1 == 1 {
        return DecodingResult{result: !(result >> 1), consumed: r.consumed}
    } else {
        return DecodingResult{result: result >> 1, consumed: r.consumed}
    }
}

pub fn decode_data(_py: Python, input: String) -> PyResult<PyList> {
    const YEAR2010: i64 = 1262304000;
    let encoded: &String = &input;
    let mut vals: Vec<i64> = vec![YEAR2010, 0, 0];
    let enc_len: u32 = encoded.len() as u32;
    let mut c: u32 = 0;
    let mut r: DecodingResult;
    let mut is_first: bool = true;
    let res: PyList = PyList::new(_py, &[]);

    while c < enc_len {
        for i in 0..3 {
            if i == 0 {
                if is_first {
                    is_first = false;
                    r = decode_signed_value_from_string(encoded, c);
                } else {
                    r = decode_unsigned_value_from_string(encoded, c);
                }
            } else {
                r = decode_signed_value_from_string(encoded, c);
            }
            c += r.consumed;
            let new_val: i64 = vals[i] + r.result;
            vals[i] = new_val;
        }
        let pt: PyDict = PyDict::new(_py);
        pt.set_item(_py, "timestamp", vals[0] as f64);
        pt.set_item(_py, "latitude", (vals[1] as f64) / 1e5);
        pt.set_item(_py, "longitude", (vals[2] as f64) / 1e5);
        res.append(_py, pt.into_object());
    }
    Ok(res)
}

fn encode_unsigned_number(num: i64) -> String  {
    let mut encoded = String::from("");
    let mut tmp: i64 = num;
    while tmp >= 0x20 {
        encoded.push_str(&(((0x20 | (tmp as u8 & 0x1f)) + 63) as char).to_string());
        tmp >>= 5;
    }
    encoded.push_str(&((tmp as u8 + 63) as char).to_string());
    return encoded;
}

fn encode_signed_number(num: i64) -> String {
    let mut sgn_num: i64 = num << 1;
    if num < 0 {
        sgn_num = !sgn_num;
    }
    return encode_unsigned_number(sgn_num);
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
            result.push_str(&encode_signed_number(tim_d));
            is_first = false;
        } else if tim_d < 0 {
            return Err(PyErr::new::<exc::ValueError, _>(_py, "invalid timestamp, list should be sorted by increasing timestamp"));
        } else {
            result.push_str(&encode_unsigned_number(tim_d));
        }

        let lat: f64 = pt.get_item(_py, "latitude").unwrap().extract(_py)?;
        let lat_d: i64 = ((lat - prev_lat) * 1e5).round() as i64;
        result.push_str(&encode_signed_number(lat_d));

        let lon: f64 = pt.get_item(_py, "longitude").unwrap().extract(_py)?;
        let lon_d: i64 = ((lon - prev_lon) * 1e5).round() as i64;
        result.push_str(&encode_signed_number(lon_d));

        prev_t += tim_d;
        prev_lat += (lat_d as f64) / 1e5;
        prev_lon += (lon_d as f64) / 1e5;
    }
    Ok(result)
}
