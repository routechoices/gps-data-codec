# polyline_encoding

Python library including base functions for polyline type encoding of gps data implemented in rust.

```
>> import polyline_encoding
>> polyline_encoding.encode_data([{'timestamp': 1628667993, 'latitude': 4.56543, 'longitude': -110.536214}])
xipk|I}tzZhbtaT
```
# Warning:
As this is for GPS data it also encodes time data, so it is not compatible with google polylines.
