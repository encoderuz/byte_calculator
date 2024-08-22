#[derive(Debug)]
pub struct ByteConverter {
    bytes: u64,
}
impl ByteConverter {
    pub fn new(bytes: u64) -> Self {
        ByteConverter { bytes }
    }

    pub fn to_kilobytes(&self) -> (f64, &'static str) {
        (self.bytes as f64 / 1024.0, "KB")
    }

    pub fn to_megabytes(&self) -> (f64, &'static str) {
        (self.bytes as f64 / 1024.0 / 1024.0, "MB")
    }

    pub fn to_gigabytes(&self) -> (f64, &'static str) {
        (self.bytes as f64 / 1024.0 / 1024.0 / 1024.0, "GB")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversion() {
        let converter = ByteConverter::new(8589934592);
        println!("{:?}", converter);
        assert_eq!(converter.to_gigabytes().0, 8.0);
    }
}