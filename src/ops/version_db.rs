//! QR-code version information database


pub const QUIRC_MAX_VERSION: usize = 40;
pub const QUIRC_MAX_ALIGNMENT: usize = 7;


pub struct QuircRsParams {
    /// Small block size
    pub bs: u32,
    /// Small data words
    pub dw: u32,
    /// Number of small blocks
    pub ns: u32,
}

pub struct QuircVersionInfo {
    pub data_bytes: u32,
    pub apat: [u32; QUIRC_MAX_ALIGNMENT],
    pub ecc: [QuircRsParams; 4],
}


pub static QUIRC_VERSION_DB: &[QuircVersionInfo] = &[QuircVersionInfo {
                                                         data_bytes: 0,
                                                         apat: [0u32; QUIRC_MAX_ALIGNMENT],
                                                         ecc: [QuircRsParams {
                                                                   bs: 0,
                                                                   dw: 0,
                                                                   ns: 0,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 0,
                                                                   dw: 0,
                                                                   ns: 0,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 0,
                                                                   dw: 0,
                                                                   ns: 0,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 0,
                                                                   dw: 0,
                                                                   ns: 0,
                                                               }],
                                                     },
                                                     QuircVersionInfo {
                                                         /* Version 1 */
                                                         data_bytes: 26,
                                                         apat: [0, 0, 0, 0, 0, 0, 0],
                                                         ecc: [QuircRsParams {
                                                                   bs: 26,
                                                                   dw: 16,
                                                                   ns: 1,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 26,
                                                                   dw: 19,
                                                                   ns: 1,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 26,
                                                                   dw: 9,
                                                                   ns: 1,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 26,
                                                                   dw: 13,
                                                                   ns: 1,
                                                               }],
                                                     },
                                                     QuircVersionInfo {
                                                         /* Version 2 */
                                                         data_bytes: 44,
                                                         apat: [6, 18, 0, 0, 0, 0, 0],
                                                         ecc: [QuircRsParams {
                                                                   bs: 44,
                                                                   dw: 28,
                                                                   ns: 1,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 44,
                                                                   dw: 34,
                                                                   ns: 1,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 44,
                                                                   dw: 16,
                                                                   ns: 1,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 44,
                                                                   dw: 22,
                                                                   ns: 1,
                                                               }],
                                                     },
                                                     QuircVersionInfo {
                                                         /* Version 3 */
                                                         data_bytes: 70,
                                                         apat: [6, 22, 0, 0, 0, 0, 0],
                                                         ecc: [QuircRsParams {
                                                                   bs: 70,
                                                                   dw: 44,
                                                                   ns: 1,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 70,
                                                                   dw: 55,
                                                                   ns: 1,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 35,
                                                                   dw: 13,
                                                                   ns: 2,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 35,
                                                                   dw: 17,
                                                                   ns: 2,
                                                               }],
                                                     },
                                                     QuircVersionInfo {
                                                         /* Version 4 */
                                                         data_bytes: 100,
                                                         apat: [6, 26, 0, 0, 0, 0, 0],
                                                         ecc: [QuircRsParams {
                                                                   bs: 50,
                                                                   dw: 32,
                                                                   ns: 2,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 100,
                                                                   dw: 80,
                                                                   ns: 1,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 25,
                                                                   dw: 9,
                                                                   ns: 4,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 50,
                                                                   dw: 24,
                                                                   ns: 2,
                                                               }],
                                                     },
                                                     QuircVersionInfo {
                                                         /* Version 5 */
                                                         data_bytes: 134,
                                                         apat: [6, 30, 0, 0, 0, 0, 0],
                                                         ecc: [QuircRsParams {
                                                                   bs: 67,
                                                                   dw: 43,
                                                                   ns: 2,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 134,
                                                                   dw: 108,
                                                                   ns: 1,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 33,
                                                                   dw: 11,
                                                                   ns: 2,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 33,
                                                                   dw: 15,
                                                                   ns: 2,
                                                               }],
                                                     },
                                                     QuircVersionInfo {
                                                         /* Version 6 */
                                                         data_bytes: 172,
                                                         apat: [6, 34, 0, 0, 0, 0, 0],
                                                         ecc: [QuircRsParams {
                                                                   bs: 43,
                                                                   dw: 27,
                                                                   ns: 4,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 86,
                                                                   dw: 68,
                                                                   ns: 2,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 43,
                                                                   dw: 15,
                                                                   ns: 4,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 43,
                                                                   dw: 19,
                                                                   ns: 4,
                                                               }],
                                                     },
                                                     QuircVersionInfo {
                                                         /* Version 7 */
                                                         data_bytes: 196,
                                                         apat: [6, 22, 38, 0, 0, 0, 0],
                                                         ecc: [QuircRsParams {
                                                                   bs: 49,
                                                                   dw: 31,
                                                                   ns: 4,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 98,
                                                                   dw: 78,
                                                                   ns: 2,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 39,
                                                                   dw: 13,
                                                                   ns: 4,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 32,
                                                                   dw: 14,
                                                                   ns: 2,
                                                               }],
                                                     },
                                                     QuircVersionInfo {
                                                         /* Version 8 */
                                                         data_bytes: 242,
                                                         apat: [6, 24, 42, 0, 0, 0, 0],
                                                         ecc: [QuircRsParams {
                                                                   bs: 60,
                                                                   dw: 38,
                                                                   ns: 2,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 121,
                                                                   dw: 97,
                                                                   ns: 2,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 40,
                                                                   dw: 14,
                                                                   ns: 4,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 40,
                                                                   dw: 18,
                                                                   ns: 4,
                                                               }],
                                                     },
                                                     QuircVersionInfo {
                                                         /* Version 9 */
                                                         data_bytes: 292,
                                                         apat: [6, 26, 46, 0, 0, 0, 0],
                                                         ecc: [QuircRsParams {
                                                                   bs: 58,
                                                                   dw: 36,
                                                                   ns: 3,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 146,
                                                                   dw: 116,
                                                                   ns: 2,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 36,
                                                                   dw: 12,
                                                                   ns: 4,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 36,
                                                                   dw: 16,
                                                                   ns: 4,
                                                               }],
                                                     },
                                                     QuircVersionInfo {
                                                         /* Version 10 */
                                                         data_bytes: 346,
                                                         apat: [6, 28, 50, 0, 0, 0, 0],
                                                         ecc: [QuircRsParams {
                                                                   bs: 69,
                                                                   dw: 43,
                                                                   ns: 4,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 86,
                                                                   dw: 68,
                                                                   ns: 2,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 43,
                                                                   dw: 15,
                                                                   ns: 6,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 43,
                                                                   dw: 19,
                                                                   ns: 6,
                                                               }],
                                                     },
                                                     QuircVersionInfo {
                                                         /* Version 11 */
                                                         data_bytes: 404,
                                                         apat: [6, 30, 54, 0, 0, 0, 0],
                                                         ecc: [QuircRsParams {
                                                                   bs: 80,
                                                                   dw: 50,
                                                                   ns: 1,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 101,
                                                                   dw: 81,
                                                                   ns: 4,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 36,
                                                                   dw: 12,
                                                                   ns: 3,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 50,
                                                                   dw: 22,
                                                                   ns: 4,
                                                               }],
                                                     },
                                                     QuircVersionInfo {
                                                         /* Version 12 */
                                                         data_bytes: 466,
                                                         apat: [6, 32, 58, 0, 0, 0, 0],
                                                         ecc: [QuircRsParams {
                                                                   bs: 58,
                                                                   dw: 36,
                                                                   ns: 6,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 116,
                                                                   dw: 92,
                                                                   ns: 2,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 42,
                                                                   dw: 14,
                                                                   ns: 7,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 46,
                                                                   dw: 20,
                                                                   ns: 4,
                                                               }],
                                                     },
                                                     QuircVersionInfo {
                                                         /* Version 13 */
                                                         data_bytes: 532,
                                                         apat: [6, 34, 62, 0, 0, 0, 0],
                                                         ecc: [QuircRsParams {
                                                                   bs: 59,
                                                                   dw: 37,
                                                                   ns: 8,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 133,
                                                                   dw: 107,
                                                                   ns: 4,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 33,
                                                                   dw: 11,
                                                                   ns: 12,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 44,
                                                                   dw: 20,
                                                                   ns: 8,
                                                               }],
                                                     },
                                                     QuircVersionInfo {
                                                         /* Version 14 */
                                                         data_bytes: 581,
                                                         apat: [6, 26, 46, 66, 0, 0, 0],
                                                         ecc: [QuircRsParams {
                                                                   bs: 64,
                                                                   dw: 40,
                                                                   ns: 4,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 145,
                                                                   dw: 115,
                                                                   ns: 3,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 36,
                                                                   dw: 12,
                                                                   ns: 11,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 36,
                                                                   dw: 16,
                                                                   ns: 11,
                                                               }],
                                                     },
                                                     QuircVersionInfo {
                                                         /* Version 15 */
                                                         data_bytes: 655,
                                                         apat: [6, 26, 48, 70, 0, 0, 0],
                                                         ecc: [QuircRsParams {
                                                                   bs: 65,
                                                                   dw: 41,
                                                                   ns: 5,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 109,
                                                                   dw: 87,
                                                                   ns: 5,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 36,
                                                                   dw: 12,
                                                                   ns: 11,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 54,
                                                                   dw: 24,
                                                                   ns: 5,
                                                               }],
                                                     },
                                                     QuircVersionInfo {
                                                         /* Version 16 */
                                                         data_bytes: 733,
                                                         apat: [6, 26, 50, 74, 0, 0, 0],
                                                         ecc: [QuircRsParams {
                                                                   bs: 73,
                                                                   dw: 45,
                                                                   ns: 7,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 122,
                                                                   dw: 98,
                                                                   ns: 5,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 45,
                                                                   dw: 15,
                                                                   ns: 3,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 43,
                                                                   dw: 19,
                                                                   ns: 15,
                                                               }],
                                                     },
                                                     QuircVersionInfo {
                                                         /* Version 17 */
                                                         data_bytes: 815,
                                                         apat: [6, 30, 54, 78, 0, 0, 0],
                                                         ecc: [QuircRsParams {
                                                                   bs: 74,
                                                                   dw: 46,
                                                                   ns: 10,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 135,
                                                                   dw: 107,
                                                                   ns: 1,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 42,
                                                                   dw: 14,
                                                                   ns: 2,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 50,
                                                                   dw: 22,
                                                                   ns: 1,
                                                               }],
                                                     },
                                                     QuircVersionInfo {
                                                         /* Version 18 */
                                                         data_bytes: 901,
                                                         apat: [6, 30, 56, 82, 0, 0, 0],
                                                         ecc: [QuircRsParams {
                                                                   bs: 69,
                                                                   dw: 43,
                                                                   ns: 9,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 150,
                                                                   dw: 120,
                                                                   ns: 5,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 42,
                                                                   dw: 14,
                                                                   ns: 2,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 50,
                                                                   dw: 22,
                                                                   ns: 17,
                                                               }],
                                                     },
                                                     QuircVersionInfo {
                                                         /* Version 19 */
                                                         data_bytes: 991,
                                                         apat: [6, 30, 58, 86, 0, 0, 0],
                                                         ecc: [QuircRsParams {
                                                                   bs: 70,
                                                                   dw: 44,
                                                                   ns: 3,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 141,
                                                                   dw: 113,
                                                                   ns: 3,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 39,
                                                                   dw: 13,
                                                                   ns: 9,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 47,
                                                                   dw: 21,
                                                                   ns: 17,
                                                               }],
                                                     },
                                                     QuircVersionInfo {
                                                         /* Version 20 */
                                                         data_bytes: 1085,
                                                         apat: [6, 34, 62, 90, 0, 0, 0],
                                                         ecc: [QuircRsParams {
                                                                   bs: 67,
                                                                   dw: 41,
                                                                   ns: 3,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 135,
                                                                   dw: 107,
                                                                   ns: 3,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 43,
                                                                   dw: 15,
                                                                   ns: 15,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 54,
                                                                   dw: 24,
                                                                   ns: 15,
                                                               }],
                                                     },
                                                     QuircVersionInfo {
                                                         /* Version 21 */
                                                         data_bytes: 1156,
                                                         apat: [6, 28, 50, 72, 92, 0, 0],
                                                         ecc: [QuircRsParams {
                                                                   bs: 68,
                                                                   dw: 42,
                                                                   ns: 17,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 144,
                                                                   dw: 116,
                                                                   ns: 4,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 46,
                                                                   dw: 16,
                                                                   ns: 19,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 50,
                                                                   dw: 22,
                                                                   ns: 17,
                                                               }],
                                                     },
                                                     QuircVersionInfo {
                                                         /* Version 22 */
                                                         data_bytes: 1258,
                                                         apat: [6, 26, 50, 74, 98, 0, 0],
                                                         ecc: [QuircRsParams {
                                                                   bs: 74,
                                                                   dw: 46,
                                                                   ns: 17,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 139,
                                                                   dw: 111,
                                                                   ns: 2,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 37,
                                                                   dw: 13,
                                                                   ns: 34,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 54,
                                                                   dw: 24,
                                                                   ns: 7,
                                                               }],
                                                     },
                                                     QuircVersionInfo {
                                                         /* Version 23 */
                                                         data_bytes: 1364,
                                                         apat: [6, 30, 54, 78, 102, 0, 0],
                                                         ecc: [QuircRsParams {
                                                                   bs: 75,
                                                                   dw: 47,
                                                                   ns: 4,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 151,
                                                                   dw: 121,
                                                                   ns: 4,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 45,
                                                                   dw: 15,
                                                                   ns: 16,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 54,
                                                                   dw: 24,
                                                                   ns: 11,
                                                               }],
                                                     },
                                                     QuircVersionInfo {
                                                         /* Version 24 */
                                                         data_bytes: 1474,
                                                         apat: [6, 28, 54, 80, 106, 0, 0],
                                                         ecc: [QuircRsParams {
                                                                   bs: 73,
                                                                   dw: 45,
                                                                   ns: 6,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 147,
                                                                   dw: 117,
                                                                   ns: 6,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 46,
                                                                   dw: 16,
                                                                   ns: 30,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 54,
                                                                   dw: 24,
                                                                   ns: 11,
                                                               }],
                                                     },
                                                     QuircVersionInfo {
                                                         /* Version 25 */
                                                         data_bytes: 1588,
                                                         apat: [6, 32, 58, 84, 110, 0, 0],
                                                         ecc: [QuircRsParams {
                                                                   bs: 75,
                                                                   dw: 47,
                                                                   ns: 8,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 132,
                                                                   dw: 106,
                                                                   ns: 8,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 45,
                                                                   dw: 15,
                                                                   ns: 22,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 54,
                                                                   dw: 24,
                                                                   ns: 7,
                                                               }],
                                                     },
                                                     QuircVersionInfo {
                                                         /* Version 26 */
                                                         data_bytes: 1706,
                                                         apat: [6, 30, 58, 86, 114, 0, 0],
                                                         ecc: [QuircRsParams {
                                                                   bs: 74,
                                                                   dw: 46,
                                                                   ns: 19,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 142,
                                                                   dw: 114,
                                                                   ns: 10,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 46,
                                                                   dw: 16,
                                                                   ns: 33,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 50,
                                                                   dw: 22,
                                                                   ns: 28,
                                                               }],
                                                     },
                                                     QuircVersionInfo {
                                                         /* Version 27 */
                                                         data_bytes: 1828,
                                                         apat: [6, 34, 62, 90, 118, 0, 0],
                                                         ecc: [QuircRsParams {
                                                                   bs: 73,
                                                                   dw: 45,
                                                                   ns: 22,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 152,
                                                                   dw: 122,
                                                                   ns: 8,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 45,
                                                                   dw: 15,
                                                                   ns: 12,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 53,
                                                                   dw: 23,
                                                                   ns: 8,
                                                               }],
                                                     },
                                                     QuircVersionInfo {
                                                         /* Version 28 */
                                                         data_bytes: 1921,
                                                         apat: [6, 26, 50, 74, 98, 122, 0],
                                                         ecc: [QuircRsParams {
                                                                   bs: 73,
                                                                   dw: 45,
                                                                   ns: 3,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 147,
                                                                   dw: 117,
                                                                   ns: 3,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 45,
                                                                   dw: 15,
                                                                   ns: 11,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 54,
                                                                   dw: 24,
                                                                   ns: 4,
                                                               }],
                                                     },
                                                     QuircVersionInfo {
                                                         /* Version 29 */
                                                         data_bytes: 2051,
                                                         apat: [6, 30, 54, 78, 102, 126, 0],
                                                         ecc: [QuircRsParams {
                                                                   bs: 73,
                                                                   dw: 45,
                                                                   ns: 21,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 146,
                                                                   dw: 116,
                                                                   ns: 7,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 45,
                                                                   dw: 15,
                                                                   ns: 19,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 53,
                                                                   dw: 23,
                                                                   ns: 1,
                                                               }],
                                                     },
                                                     QuircVersionInfo {
                                                         /* Version 30 */
                                                         data_bytes: 2185,
                                                         apat: [6, 26, 52, 78, 104, 130, 0],
                                                         ecc: [QuircRsParams {
                                                                   bs: 75,
                                                                   dw: 47,
                                                                   ns: 19,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 145,
                                                                   dw: 115,
                                                                   ns: 5,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 45,
                                                                   dw: 15,
                                                                   ns: 23,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 54,
                                                                   dw: 24,
                                                                   ns: 15,
                                                               }],
                                                     },
                                                     QuircVersionInfo {
                                                         /* Version 31 */
                                                         data_bytes: 2323,
                                                         apat: [6, 30, 56, 82, 108, 134, 0],
                                                         ecc: [QuircRsParams {
                                                                   bs: 74,
                                                                   dw: 46,
                                                                   ns: 2,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 145,
                                                                   dw: 115,
                                                                   ns: 13,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 45,
                                                                   dw: 15,
                                                                   ns: 23,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 54,
                                                                   dw: 24,
                                                                   ns: 42,
                                                               }],
                                                     },
                                                     QuircVersionInfo {
                                                         /* Version 32 */
                                                         data_bytes: 2465,
                                                         apat: [6, 34, 60, 86, 112, 138, 0],
                                                         ecc: [QuircRsParams {
                                                                   bs: 74,
                                                                   dw: 46,
                                                                   ns: 10,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 145,
                                                                   dw: 115,
                                                                   ns: 17,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 45,
                                                                   dw: 15,
                                                                   ns: 19,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 54,
                                                                   dw: 24,
                                                                   ns: 10,
                                                               }],
                                                     },
                                                     QuircVersionInfo {
                                                         /* Version 33 */
                                                         data_bytes: 2611,
                                                         apat: [6, 30, 58, 86, 114, 142, 0],
                                                         ecc: [QuircRsParams {
                                                                   bs: 74,
                                                                   dw: 46,
                                                                   ns: 14,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 145,
                                                                   dw: 115,
                                                                   ns: 17,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 45,
                                                                   dw: 15,
                                                                   ns: 11,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 54,
                                                                   dw: 24,
                                                                   ns: 29,
                                                               }],
                                                     },
                                                     QuircVersionInfo {
                                                         /* Version 34 */
                                                         data_bytes: 2761,
                                                         apat: [6, 34, 62, 90, 118, 146, 0],
                                                         ecc: [QuircRsParams {
                                                                   bs: 74,
                                                                   dw: 46,
                                                                   ns: 14,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 145,
                                                                   dw: 115,
                                                                   ns: 13,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 46,
                                                                   dw: 16,
                                                                   ns: 59,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 54,
                                                                   dw: 24,
                                                                   ns: 44,
                                                               }],
                                                     },
                                                     QuircVersionInfo {
                                                         /* Version 35 */
                                                         data_bytes: 2876,
                                                         apat: [6, 30, 54, 78, 102, 126, 150],
                                                         ecc: [QuircRsParams {
                                                                   bs: 75,
                                                                   dw: 47,
                                                                   ns: 12,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 151,
                                                                   dw: 121,
                                                                   ns: 12,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 45,
                                                                   dw: 15,
                                                                   ns: 22,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 54,
                                                                   dw: 24,
                                                                   ns: 39,
                                                               }],
                                                     },
                                                     QuircVersionInfo {
                                                         /* Version 36 */
                                                         data_bytes: 3034,
                                                         apat: [6, 24, 50, 76, 102, 128, 154],
                                                         ecc: [QuircRsParams {
                                                                   bs: 75,
                                                                   dw: 47,
                                                                   ns: 6,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 151,
                                                                   dw: 121,
                                                                   ns: 6,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 45,
                                                                   dw: 15,
                                                                   ns: 2,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 54,
                                                                   dw: 24,
                                                                   ns: 46,
                                                               }],
                                                     },
                                                     QuircVersionInfo {
                                                         /* Version 37 */
                                                         data_bytes: 3196,
                                                         apat: [6, 28, 54, 80, 106, 132, 158],
                                                         ecc: [QuircRsParams {
                                                                   bs: 74,
                                                                   dw: 46,
                                                                   ns: 29,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 152,
                                                                   dw: 122,
                                                                   ns: 17,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 45,
                                                                   dw: 15,
                                                                   ns: 24,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 54,
                                                                   dw: 24,
                                                                   ns: 49,
                                                               }],
                                                     },
                                                     QuircVersionInfo {
                                                         /* Version 38 */
                                                         data_bytes: 3362,
                                                         apat: [6, 32, 58, 84, 110, 136, 162],
                                                         ecc: [QuircRsParams {
                                                                   bs: 74,
                                                                   dw: 46,
                                                                   ns: 13,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 152,
                                                                   dw: 122,
                                                                   ns: 4,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 45,
                                                                   dw: 15,
                                                                   ns: 42,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 54,
                                                                   dw: 24,
                                                                   ns: 48,
                                                               }],
                                                     },
                                                     QuircVersionInfo {
                                                         /* Version 39 */
                                                         data_bytes: 3532,
                                                         apat: [6, 26, 54, 82, 110, 138, 166],
                                                         ecc: [QuircRsParams {
                                                                   bs: 75,
                                                                   dw: 47,
                                                                   ns: 40,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 147,
                                                                   dw: 117,
                                                                   ns: 20,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 45,
                                                                   dw: 15,
                                                                   ns: 10,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 54,
                                                                   dw: 24,
                                                                   ns: 43,
                                                               }],
                                                     },
                                                     QuircVersionInfo {
                                                         /* Version 40 */
                                                         data_bytes: 3706,
                                                         apat: [6, 30, 58, 86, 114, 142, 170],
                                                         ecc: [QuircRsParams {
                                                                   bs: 75,
                                                                   dw: 47,
                                                                   ns: 18,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 148,
                                                                   dw: 118,
                                                                   ns: 19,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 45,
                                                                   dw: 15,
                                                                   ns: 20,
                                                               },
                                                               QuircRsParams {
                                                                   bs: 54,
                                                                   dw: 24,
                                                                   ns: 34,
                                                               }],
                                                     }];
