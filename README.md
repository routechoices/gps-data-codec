[![PyPi version](https://pypip.in/v/$REPO/badge.png)](https://crate.io/packages/$REPO/)
[![PyPi downloads](https://pypip.in/d/$REPO/badge.png)](https://crate.io/packages/$REPO/)


# gps_encoding

Python library, implemented in rust, including base functions for encoding of gps data.  
It is used to encode competitors data on https://www.routechoices.com

```
>> import gps_encoding
>> gps_encoding.encode_data([(1628667993, 4.56543, -110.536214), ]) # [(time, lat, lon), ...]
'qtaxyT}tzZhbtaT'
```

# Warning:  
  - The list of GPS points must be sorted by increasing timestamps.
