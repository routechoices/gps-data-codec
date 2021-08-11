# polyline_encoding

Python library, implemented in rust, including base functions for encoding of gps data.  
It is used to encode competitors data on https://www.routechoices.com

```
>> import polyline_encoding
>> polyline_encoding.encode_data([{'timestamp': 1628667993, 'latitude': 4.56543, 'longitude': -110.536214}])
xipk|I}tzZhbtaT
```

# Warning:
  - As this is for GPS data it also encodes time data, so it is not compatible with google polylines.  
  - Timestamp must be after 1st of January 2010, and the list must be sorted by increasing timestamps.
