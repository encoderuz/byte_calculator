/// `ByteConverter` is a structure for converting a given byte size into different units (KB, MB, GB, TB).
///
/// This structure and its methods allow you to convert a size in bytes into kilobytes, megabytes, gigabytes,
/// and terabytes, as well as perform conversions back to bytes.
pub struct ByteConverter {
    pub bytes: u64,
}

impl ByteConverter {
    /// The constant value for one kilobyte (1024.0).
    pub const KB: f64 = 1024.0;
    /// The constant value for one megabyte (1024.0 * KB).
    pub const MB: f64 = 1024.0 * Self::KB;
    /// The constant value for one gigabyte (1024.0 * MB).
    pub const GB: f64 = 1024.0 * Self::MB;
    /// The constant value for one terabyte (1024.0 * GB).
    pub const TB: f64 = 1024.0 * Self::GB;

    /// Creates a new `ByteConverter` instance.
    ///
    /// # Parameters
    ///
    /// - `bytes`: The number of bytes to be converted.
    ///
    /// # Example
    ///
    /// ```
    /// use data_storage_units::ByteConverter;
    /// let converter = ByteConverter::new(1024);
    /// ```
    pub fn new(bytes: u64) -> Self {
        ByteConverter { bytes }
    }

    /// Converts bytes to kilobytes (KB).
    ///
    /// # Returns
    ///
    /// A tuple containing the value in kilobytes as `f64` and the unit "KB".
    ///
    /// # Example
    ///
    /// ```
    /// use data_storage_units::ByteConverter;
    /// let converter = ByteConverter::new(1024);
    /// let (value, unit) = converter.to_kb();
    /// assert_eq!(value, 1.0);
    /// assert_eq!(unit, "KB");
    /// ```
    pub fn to_kb(&self) -> (f64, &'static str) {
        (self.bytes as f64 / Self::KB, "KB")
    }

    /// Converts bytes to megabytes (MB).
    ///
    /// # Returns
    ///
    /// A tuple containing the value in megabytes as `f64` and the unit "MB".
    ///
    /// # Example
    ///
    /// ```
    /// use data_storage_units::ByteConverter;
    /// let converter = ByteConverter::new(1073741824);
    /// let (value, unit) = converter.to_mb();
    /// assert_eq!(value, 1024.0);
    /// assert_eq!(unit, "MB");
    /// ```
    pub fn to_mb(&self) -> (f64, &'static str) {
        (self.bytes as f64 / Self::MB, "MB")
    }

    /// Converts bytes to gigabytes (GB).
    ///
    /// # Returns
    ///
    /// A tuple containing the value in gigabytes as `f64` and the unit "GB".
    ///
    /// # Example
    ///
    /// ```
    /// use data_storage_units::ByteConverter;
    /// let converter = ByteConverter::new(1073741824);
    /// let (value, unit) = converter.to_gb();
    /// assert_eq!(value, 1.0);
    /// assert_eq!(unit, "GB");
    /// ```
    pub fn to_gb(&self) -> (f64, &'static str) {
        (self.bytes as f64 / Self::GB, "GB")
    }

    /// Converts bytes to terabytes (TB).
    ///
    /// # Returns
    ///
    /// A tuple containing the value in terabytes as `f64` and the unit "TB".
    ///
    /// # Example
    ///
    /// ```
    /// use data_storage_units::ByteConverter;
    /// let converter = ByteConverter::new(1073741824);
    /// let (value, unit) = converter.to_tb();
    /// assert_eq!(value, 0.0009765625);
    /// assert_eq!(unit, "TB");
    /// ```
    pub fn to_tb(&self) -> (f64, &'static str) {
        (self.bytes as f64 / Self::TB, "TB")
    }

    #[cfg(test)]
    fn test_to_kb_conversion() {
        let converter = ByteConverter::new(8589934592);
        assert_eq!(converter.to_kb().0, 8388608.0);
    }

    #[cfg(test)]
    fn test_to_mb_conversion() {
        let converter = ByteConverter::new(8589934592);
        assert_eq!(converter.to_mb().0, 8192.0);
    }

    #[cfg(test)]
    fn test_to_gb_conversion() {
        let converter = ByteConverter::new(8589934592);
        assert_eq!(converter.to_gb().0, 8.0);
    }

    #[cfg(test)]
    fn test_to_tb_conversion() {
        let converter = ByteConverter::new(8589934592);
        assert_eq!(converter.to_tb().0, 0.0078125);
    }
}