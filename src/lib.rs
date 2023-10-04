use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use pyo3::types::{PyTuple, PyList};

struct DecodingResult {
    value: i64,
    consumed: u32,
}

fn decode_unsigned_value_from_string(encoded: &[u8], offset: u32, encoded_length: u32) -> DecodingResult {
    let mut value: i64 = 0;
    let mut consumed: u32 = 0;
    let mut size: u32 = 0;
    let mut byte: u8 = 0x20;
    while byte >= 0x20 && consumed + offset < encoded_length {
        assert!(consumed + offset < encoded_length);
        byte = encoded[(consumed + offset) as usize] - 63;
        consumed += 1;
        value |=  ((byte & 0x1f) as i64) << size;
        size += 5;
    }
    return DecodingResult{value: value, consumed: consumed};
}

fn decode_signed_value_from_string(encoded: &[u8], offset: u32, encoded_length: u32) -> DecodingResult {
    let tmp_result: DecodingResult = decode_unsigned_value_from_string(encoded, offset, encoded_length);
    let tmp_value: i64 = tmp_result.value;
    if tmp_value & 1 == 1 {
        return DecodingResult{
            value: !(tmp_value >> 1),
            consumed: tmp_result.consumed
        }
    } else {
        return DecodingResult{
            value: tmp_value >> 1,
            consumed: tmp_result.consumed
        }
    }
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
fn decode(_py: Python, input: String) -> PyResult<PyObject> {    
    const YEAR2010: i64 = 1262304000;
    let mut values: [i64; 3] = [YEAR2010, 0, 0];
    let mut bytes_consumed: u32 = 0;
    let mut decoding_result: DecodingResult;
    let encoded: &[u8] = &input.as_bytes();
    let encoded_length: u32 = encoded.len() as u32;
    let output: &PyList = PyList::empty(_py);
    while bytes_consumed < encoded_length {
        for i in 0..3 {
            if i == 0 {
                if bytes_consumed == 0 {
                    decoding_result = decode_signed_value_from_string(encoded, bytes_consumed, encoded_length);
                } else {
                    decoding_result = decode_unsigned_value_from_string(encoded, bytes_consumed, encoded_length);
                }
            } else {
                decoding_result = decode_signed_value_from_string(encoded, bytes_consumed, encoded_length);
            }
            values[i] += decoding_result.value;
            bytes_consumed += decoding_result.consumed;
        }
        output.append(
            PyTuple::new(
                _py,
                [
                    (values[0] as i64).to_object(_py),
                    ((values[1] as f64) / 1e5).to_object(_py),
                    ((values[2] as f64) / 1e5).to_object(_py)
                ]
            )
        )?;
    }
    Ok(output.to_object(_py))
}

#[pyfunction]
fn encode(_py: Python, data: &PyList) -> PyResult<String> {
    const YEAR2010: i64 = 1262304000;
    let mut prev_timestamp: i64 = YEAR2010;
    let mut prev_latitude: f64 = 0.0;
    let mut prev_longitude: f64 = 0.0;

    let mut output: Vec<u8> = vec![];

    for (point_count, point_object) in data.iter().enumerate() {
        let point_data: &PyTuple = point_object.downcast::<PyTuple>()?;
        if point_data.len() != 3 {
            return Err(PyValueError::new_err("invalid list, item does not contains a valid GPS data array"));
        }

        let timestamp: f64 = point_data[0].extract::<f64>()?;
        let timestamp_diff: i64 = timestamp.round() as i64 - prev_timestamp;
        if point_count == 0 {
            output.append(&mut encode_signed_number(timestamp_diff));
        } else if timestamp_diff >= 0 {
            output.append(&mut encode_unsigned_number(timestamp_diff));
        } else {
            return Err(PyValueError::new_err("invalid timestamp, list should be sorted by increasing timestamp"));
        }

        let latitude: f64 = point_data[1].extract::<f64>()?;
        let latitude_diff: i64 = ((latitude - prev_latitude) * 1e5).round() as i64;
        output.append(&mut encode_signed_number(latitude_diff));

        let longitude: f64 = point_data[2].extract::<f64>()?;
        let longitude_diff: i64 = ((longitude - prev_longitude) * 1e5).round() as i64;
        output.append(&mut encode_signed_number(longitude_diff));

        prev_timestamp += timestamp_diff;
        prev_latitude += (latitude_diff as f64) / 1e5;
        prev_longitude += (longitude_diff as f64) / 1e5;
    }
    Ok(String::from_utf8(output).unwrap())
}

#[pymodule]
fn gps_data_codec(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(encode, m)?)?;
    m.add_function(wrap_pyfunction!(decode, m)?)?;
    Ok(())
}