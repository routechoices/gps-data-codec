use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use pyo3::types::{PyTuple, PyList};

#[pymodule]
fn gps_data_codec(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(encode, m)?)?;
    m.add_function(wrap_pyfunction!(decode, m)?)?;
    Ok(())
}

struct DecodingResult {
    result: i64,
    consumed: u32,
}

fn decode_unsigned_value_from_string(encoded: &[u8], offset: u32) -> DecodingResult {
    let enc_len: u32 = encoded.len() as u32;
    let mut i: u32 = 0;
    let mut s: u32 = 0;
    let mut result: i64 = 0;
    let mut b: u8 = 0x20;
    while b >= 0x20 && i + offset < enc_len {
        assert!(i + offset < enc_len);
        b = encoded[(i + offset) as usize] - 63;
        i += 1;
        result |=  ((b & 0x1f) as i64) << s;
        s += 5;
    }
    return DecodingResult{result: result, consumed: i};
}

fn decode_signed_value_from_string(encoded: &[u8], offset: u32) -> DecodingResult {
    let r: DecodingResult = decode_unsigned_value_from_string(encoded, offset);
    let result: i64 = r.result;
    if result & 1 == 1 {
        return DecodingResult{result: !(result >> 1), consumed: r.consumed}
    } else {
        return DecodingResult{result: result >> 1, consumed: r.consumed}
    }
}

#[pyfunction]
fn decode(_py: Python, input: String) -> PyResult<PyObject> {    
    const YEAR2010: i64 = 1262304000;
    let encoded: &[u8] = &input.as_bytes();
    let mut vals: Vec<i64> = vec![YEAR2010, 0, 0];
    let enc_len: u32 = encoded.len() as u32;
    let mut c: u32 = 0;
    let mut r: DecodingResult;
    let mut is_first: bool = true;
    let res: &PyList = PyList::empty(_py);
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
        let pt: &PyTuple = PyTuple::new(_py, [(vals[0] as i64).to_object(_py), ((vals[1] as f64) / 1e5).to_object(_py), ((vals[2] as f64) / 1e5).to_object(_py)]);
        res.append(pt)?;
    }
    Ok(res.to_object(_py))
}

fn encode_unsigned_number(num: i64) -> Vec<u8>  {
    let mut encoded: Vec<u8> = vec![];
    let mut tmp: i64 = num;
    while tmp >= 0x20 {
        encoded.push(((0x20 | (tmp as u8 & 0x1f)) + 63) as u8);
        tmp >>= 5;
    }
    encoded.push((tmp as u8 + 63) as u8);
    return encoded;
}

fn encode_signed_number(num: i64) -> Vec<u8> {
    let mut sgn_num: i64 = num << 1;
    if num < 0 {
        sgn_num = !sgn_num;
    }
    return encode_unsigned_number(sgn_num);
}

#[pyfunction]
fn encode(_py: Python, data: &PyList) -> PyResult<String> {
    const YEAR2010: i64 = 1262304000;
    let mut prev_t: i64 = YEAR2010;
    let mut prev_lat: f64 = 0.0;
    let mut prev_lon: f64 = 0.0;

    let mut result: Vec<u8> = vec![];
    
    let mut is_first: bool = true;
    
    for py_pt in data.iter() {
        let pt: &PyTuple = py_pt.downcast::<PyTuple>()?;
        if pt.len() != 3 {
            return Err(PyValueError::new_err("invalid list, item does not contains a valid GPS data array"));
        }

        let tim: f64 = pt[0].extract::<f64>()?;
        let tim_d: i64 = tim.round() as i64 - prev_t;
        if is_first {
            result.append(&mut encode_signed_number(tim_d));
            is_first = false;
        } else if tim_d < 0 {
            return Err(PyValueError::new_err("invalid timestamp, list should be sorted by increasing timestamp"));
        } else {
            result.append(&mut encode_unsigned_number(tim_d));
        }

        let lat: f64 = pt[1].extract::<f64>()?;
        let lat_d: i64 = ((lat - prev_lat) * 1e5).round() as i64;
        result.append(&mut encode_signed_number(lat_d));

        let lon: f64 = pt[2].extract::<f64>()?;
        let lon_d: i64 = ((lon - prev_lon) * 1e5).round() as i64;
        result.append(&mut encode_signed_number(lon_d));

        prev_t += tim_d;
        prev_lat += (lat_d as f64) / 1e5;
        prev_lon += (lon_d as f64) / 1e5;
    }
    Ok(String::from_utf8(result).unwrap())
}
