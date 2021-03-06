use self::super::constants::{QUIRC_MAX_PAYLOAD, QUIRC_MAX_BITMAP};
use self::super::ops::{QuircPoint, QuircCode, QuircData, Quirc};
use std::boxed::Box;
use libc::c_int;
use std::ptr;


/// Version of `Quirc` for FFI consumption.
#[repr(C)]
pub struct FfiQuirc {
    inner: *mut Quirc,
}

impl FfiQuirc {
    pub fn new(from: Quirc) -> FfiQuirc {
        FfiQuirc { inner: Box::into_raw(Box::new(from)) }
    }
}

impl Drop for FfiQuirc {
    fn drop(&mut self) {
        if !self.inner.is_null() {
            unsafe {
                Box::from_raw(self.inner);
            }
            self.inner = ptr::null_mut();
        }
    }
}

/// This structure describes a location in the input image buffer.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FfiQuircPoint {
    pub x: c_int,
    pub y: c_int,
}

impl From<QuircPoint> for FfiQuircPoint {
    fn from(code: QuircPoint) -> FfiQuircPoint {
        FfiQuircPoint {
            x: code.x as c_int,
            y: code.y as c_int,
        }
    }
}


/// This structure is used to return information about detected QR codes
/// in the input image — FFI.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FfiQuircCode {
    /// The four corners of the QR-code, from top left, clockwise
    pub corners: [FfiQuircPoint; 4],

    /// The number of cells across in the QR-code.
    pub size: c_int,
    /// The cell bitmap is a bitmask giving the actual values of cells.
    ///
    /// If the cell at (x, y) is black, then the following bit is set:
    ///
    /// ```c
    /// cell_bitmap[i >> 3] & (1 << (i & 7))
    /// ```
    ///
    /// where `i = (y * size) + x`.
    pub cell_bitmap: [u8; QUIRC_MAX_BITMAP],
}

impl From<QuircCode> for FfiQuircCode {
    fn from(code: QuircCode) -> FfiQuircCode {
        FfiQuircCode {
            corners: [code.corners[0].into(), code.corners[1].into(), code.corners[2].into(), code.corners[3].into()],
            size: code.size as c_int,
            cell_bitmap: code.cell_bitmap,
        }
    }
}


/// This structure holds the decoded QR-code data
#[repr(C)]
pub struct FfiQuircData {
    /// Various parameters of the QR-code. These can mostly be
    /// ignored if you only care about the data.
    pub version: c_int,
    pub ecc_level: c_int,
    pub mask: c_int,

    /// This field is the highest-valued data type found in the QR
    /// code.
    pub data_type: c_int,

    /// Data payload. For the Kanji datatype, payload is encoded as
    /// Shift-JIS. For all other datatypes, payload is ASCII text.
    pub payload: [u8; QUIRC_MAX_PAYLOAD],
    pub payload_len: c_int,

    /// ECI assignment number
    pub eci: u32,
}

impl From<QuircData> for FfiQuircData {
    fn from(data: QuircData) -> FfiQuircData {
        FfiQuircData {
            version: data.version as c_int,
            ecc_level: data.ecc_level as c_int,
            mask: data.mask as c_int,
            data_type: data.data_type as c_int,
            payload: data.payload,
            payload_len: data.payload_len as c_int,
            eci: data.eci,
        }
    }
}


/// This enum describes the various decoder errors which may occur.
#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum QuircDecodeError {
    QuircSuccess = 0,
    QuircErrorInvalidGridSize,
    QuircErrorInvalidVersion,
    QuircErrorFormatEcc,
    QuircErrorDataEcc,
    QuircErrorUnknownDataType,
    QuircErrorDataOverflow,
    QuircErrorDataUnderflow,
}


/// Obtain the library version string.
#[no_mangle]
pub extern "C" fn quirc_version() -> *const u8 {
    concat!(env!("CARGO_PKG_VERSION"), "\0").as_ptr()
}

/// Construct a new QR-code recognizer.
///
/// This function will return a null pointer if sufficient memory could not be allocated.
#[no_mangle]
pub extern "C" fn quirc_new() -> *mut FfiQuirc {
    Box::into_raw(Box::new(FfiQuirc::new(Quirc::new())))
}

/// Destroy a QR-code recognizer.
#[no_mangle]
pub unsafe extern "C" fn quirc_destroy(whom: *mut FfiQuirc) {
    Box::from_raw(whom);
}

/// Resize the QR-code recognizer. The size of an image must be
/// specified before codes can be analyzed.
///
/// This function returns 0 on success, or -1 if sufficient memory could
/// not be allocated.
#[no_mangle]
pub unsafe extern "C" fn quirc_resize(whom: *mut FfiQuirc, w: c_int, h: c_int) -> c_int {
    (*((*whom).inner)).resize(w as usize, h as usize);
    0
}

/// These functions are used to process images for QR-code recognition.
/// `quirc_begin()` must first be called to obtain access to a buffer into
/// which the input image should be placed. Optionally, the current
/// width and height may be returned.
#[no_mangle]
pub unsafe extern "C" fn quirc_begin(whom: *mut FfiQuirc, out_w: *mut c_int, out_h: *mut c_int) -> *mut u8 {
    if !out_w.is_null() || !out_h.is_null() {
        let (w, h) = (*((*whom).inner)).size();

        if !out_w.is_null() {
            *out_w = w as c_int;
        }
        if !out_h.is_null() {
            *out_h = h as c_int;
        }
    }

    (*((*whom).inner)).begin().as_mut_ptr()
}

/// After filling the buffer, `quirc_end()` should be called to process
/// the image for QR-code recognition. The locations and content of each
/// code may be obtained using accessor functions described below.
#[no_mangle]
pub unsafe extern "C" fn quirc_end(whom: *mut FfiQuirc) {
    (*((*whom).inner)).end()
}

/// Return a string error message for an error code.
#[no_mangle]
pub extern "C" fn quirc_strerror(err: QuircDecodeError) -> *const u8 {
    #[allow(unreachable_patterns)]
    match err {
            QuircDecodeError::QuircSuccess => "Success\0",
            QuircDecodeError::QuircErrorInvalidGridSize => "Invalid grid size\0",
            QuircDecodeError::QuircErrorInvalidVersion => "Invalid version\0",
            QuircDecodeError::QuircErrorFormatEcc => "Format data ECC failure\0",
            QuircDecodeError::QuircErrorDataEcc => "ECC failure\0",
            QuircDecodeError::QuircErrorUnknownDataType => "Unknown data type\0",
            QuircDecodeError::QuircErrorDataOverflow => "Data overflow\0",
            QuircDecodeError::QuircErrorDataUnderflow => "Data underflow\0",
            _ => "Unknown error\0",
        }
        .as_ptr()
}

/// Return the number of QR-codes identified in the last processed image.
#[no_mangle]
pub unsafe extern "C" fn quirc_count(in_whom: *const FfiQuirc) -> c_int {
    (*((*in_whom).inner)).count() as c_int
}

/// Extract the QR-code specified by the given index.
#[no_mangle]
pub unsafe extern "C" fn quirc_extract(from_whom: *const FfiQuirc, index: c_int, code: *mut FfiQuircCode) {
    if index < 0 {
        return;
    }

    if let Some(out) = (*((*from_whom).inner)).extract(index as usize) {
        *code = out.into();
    }
}

/// Decode a QR-code, returning the payload data.
#[no_mangle]
pub unsafe extern "C" fn quirc_decode(code: *const FfiQuircCode, data: *mut FfiQuircData) -> QuircDecodeError {
    let mut out_data = QuircData {
        version: 0,
        ecc_level: 0,
        mask: 0,

        data_type: 0,

        payload: [0u8; QUIRC_MAX_PAYLOAD],
        payload_len: 0,

        eci: 0,
    };

    let err = QuircCode::from(*code).decode(&mut out_data);
    *data = out_data.into();

    err
}
