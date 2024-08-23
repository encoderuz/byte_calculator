use super::ByteConverter;
impl ByteConverter {
    pub fn new(bytes: u64) -> Self {
        ByteConverter { bytes }
    }
    pub fn to_kb(&self) -> (f64, &'static str) {
        (self.bytes as f64 / 1024.0, "KB")
    }

    pub fn to_mb(&self) -> (f64, &'static str) {
        (self.bytes as f64 / 1024.0 / 1024.0, "MB")
    }

    pub fn to_gb(&self) -> (f64, &'static str) {
        (self.bytes as f64 / 1024.0 / 1024.0 / 1024.0, "GB")
    }

    pub fn to_tb(&self) -> (f64, &'static str) {
        (self.bytes as f64 / 1024.0 / 1024.0 / 1024.0 / 1024.0, "TB")
    }
    pub fn tb_to_byte(tb: f64) -> Self{
        ByteConverter{
            bytes: (tb * 1024.0 * 1024.0 * 1024.0 * 1024.0) as u64
        }
    }
}