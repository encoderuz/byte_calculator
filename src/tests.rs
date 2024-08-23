#[cfg(test)]
mod tests {
    use crate::byte_converter::ByteConverter;

    #[test]
    fn test_to_kb_conversion() {
        let converter = ByteConverter::new(8589934592);
        assert_eq!(converter.to_kb().0, 8388608.0);
    }

    #[test]
    fn test_to_mb_conversion() {
        let converter = ByteConverter::new(8589934592);
        assert_eq!(converter.to_mb().0, 8192.0);
    }

    #[test]
    fn test_to_gb_conversion() {
        let converter = ByteConverter::new(8589934592);
        assert_eq!(converter.to_gb().0, 8.0);
    }

    #[test]
    fn test_to_tb_conversion() {
        let converter = ByteConverter::new(8589934592);
        assert_eq!(converter.to_tb().0, 0.0078125);
    }
}