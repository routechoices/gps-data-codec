# gps_encoding

Python library, implemented in rust, including base functions for encoding of gps data.  
It is used to encode competitors data on https://www.routechoices.com

```
>> import gps_encoding
>> gps_encoding.encode_data([{'timestamp': 1628667993, 'latitude': 4.56543, 'longitude': -110.536214}])
'qtaxyT}tzZhbtaT'
```

# Warning:  
  - The list of GPS points must be sorted by increasing timestamps.
