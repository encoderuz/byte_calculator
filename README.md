# Unit Converter for Rust Language
![Test](https://github.com/encoderuz/unit_converter/actions/workflows/rust.yml/badge.svg)
### This library allows you to convert a size in bytes into kilobytes, megabytes, gigabytes, and terabytes, as well as perform conversions back to bytes.


### Create a new `ByteConverter` instance
`bytes`: The number of bytes to be converted.
```rust
use unit_converter::ByteConverter;
let converter = ByteConverter::new(1024);
```
### Converts bytes to kilobytes (KB)
A tuple containing the value in kilobytes as `f64` and the unit "KB".
```rust
let converter = ByteConverter::new(1024);
let (value, unit) = converter.to_kb();
```
### Converts bytes to megabytes (MB)
A tuple containing the value in megabytes as `f64` and the unit "MB".
```rust
let converter = ByteConverter::new(1073741824);
let (value, unit) = converter.to_mb();
```
### Converts bytes to gigabytes (GB)
A tuple containing the value in gigabytes as `f64` and the unit "GB".
```rust
let converter = ByteConverter::new(1073741824);
/let (value, unit) = converter.to_gb();
```
### Converts bytes to terabytes (TB)
A tuple containing the value in terabytes as `f64` and the unit "TB".
```rust
let converter = ByteConverter::new(1073741824);
let (value, unit) = converter.to_tb();
```