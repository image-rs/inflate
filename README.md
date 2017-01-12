# inflate
A [DEFLATE](http://www.gzip.org/zlib/rfc-deflate.html) decoder written in rust.

This library provides functionality to decompress data compressed with the DEFLATE algorithm,
both with and without a [zlib](https://tools.ietf.org/html/rfc1950) header/trailer.

# Examples:
```rust
    use inflate::InflateStream;

    let data = [0x73, 0x49, 0x4d, 0xcb, 0x49, 0x2c, 0x49, 0x55, 0x00, 0x11, 0x00];
    let mut inflater = InflateStream::new();
    let mut out = Vec::<u8>::new();
    let mut n = 0;
    while n < data.len() {
        let res = inflater.update(&data[n..]);
        if let Ok((num_bytes_read, result)) = res {
            n += num_bytes_read;
            out.extend(result);
        } else {
            res.unwrap();
        }
    }
```
