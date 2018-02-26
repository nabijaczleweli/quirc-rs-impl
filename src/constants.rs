/// Limits on the maximum size of QR-codes and their content.
pub const QUIRC_MAX_BITMAP: usize = 3917;
pub const QUIRC_MAX_PAYLOAD: usize = 8896;

/// QR-code ECC types.
pub const QUIRC_ECC_LEVEL_M: u8 = 0;
pub const QUIRC_ECC_LEVEL_L: u8 = 1;
pub const QUIRC_ECC_LEVEL_H: u8 = 2;
pub const QUIRC_ECC_LEVEL_Q: u8 = 3;

/// QR-code data types.
pub const QUIRC_DATA_TYPE_NUMERIC: u8 = 1;
pub const QUIRC_DATA_TYPE_ALPHA: u8 = 2;
pub const QUIRC_DATA_TYPE_BYTE: u8 = 4;
pub const QUIRC_DATA_TYPE_KANJI: u8 = 8;

/// Common character encodings
pub const QUIRC_ECI_ISO_8859_1: u8 = 1;
pub const QUIRC_ECI_IBM437: u8 = 2;
pub const QUIRC_ECI_ISO_8859_2: u8 = 4;
pub const QUIRC_ECI_ISO_8859_3: u8 = 5;
pub const QUIRC_ECI_ISO_8859_4: u8 = 6;
pub const QUIRC_ECI_ISO_8859_5: u8 = 7;
pub const QUIRC_ECI_ISO_8859_6: u8 = 8;
pub const QUIRC_ECI_ISO_8859_7: u8 = 9;
pub const QUIRC_ECI_ISO_8859_8: u8 = 10;
pub const QUIRC_ECI_ISO_8859_9: u8 = 11;
pub const QUIRC_ECI_WINDOWS_874: u8 = 13;
pub const QUIRC_ECI_ISO_8859_13: u8 = 15;
pub const QUIRC_ECI_ISO_8859_15: u8 = 17;
pub const QUIRC_ECI_SHIFT_JIS: u8 = 20;
pub const QUIRC_ECI_UTF_8: u8 = 26;
