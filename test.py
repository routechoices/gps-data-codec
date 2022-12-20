import gps_encoding


def test_lib():
    input = [(-1,0,0),(1628667993, 4.56543, -110.53621), (1628667994, 4.56553, -110.53625)]
    expected_encoded = '`o|sfjA??ya_fpo@}tzZhbtaT@SF'
    encoded = gps_encoding.encode_data(input)
    assert(encoded == expected_encoded)
    output = gps_encoding.decode_data(encoded)
    assert(output == input)


if __name__ == "__main__":
    test_lib()