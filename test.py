import gps_encoding


def test_lib():
    input = [(1628667993.0, 4.56543, -110.53621), (1628667994.0, 4.56553, -110.53625)]
    expected_encoded = 'qtaxyT}tzZhbtaT@SF'
    encoded = gps_encoding.encode_data(input)
    assert(encoded == expected_encoded)
    output = gps_encoding.decode_data(encoded)
    assert(output == input)


if __name__ == "__main__":
    test_lib()