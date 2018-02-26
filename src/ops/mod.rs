pub mod version_db;

use self::super::ffi::{FfiQuircPoint, FfiQuircCode, FfiQuircData};
use self::super::constants::{QUIRC_MAX_PAYLOAD, QUIRC_MAX_BITMAP};
use std::{hash, cmp, fmt, u16};


pub(crate) const QUIRC_MAX_REGIONS: usize = 256;
pub(crate) const QUIRC_MAX_CAPSTONES: usize = 32;
pub(crate) const QUIRC_MAX_GRIDS: usize = 8;
pub(crate) const QUIRC_PERSPECTIVE_PARAMS: usize = 8;


/// This structure describes a location in the input image buffer.
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct QuircPoint {
    pub x: isize,
    pub y: isize,
}

impl From<FfiQuircPoint> for QuircPoint {
    fn from(code: FfiQuircPoint) -> QuircPoint {
        QuircPoint {
            x: code.x as isize,
            y: code.y as isize,
        }
    }
}


/// This structure is used to return information about detected QR codes
/// in the input image.
#[derive(Copy, Clone)]
pub struct QuircCode {
    /// The four corners of the QR-code, from top left, clockwise
    pub corners: [QuircPoint; 4],

    /// The number of cells across in the QR-code.
    pub size: u32,
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

impl From<FfiQuircCode> for QuircCode {
    fn from(code: FfiQuircCode) -> QuircCode {
        QuircCode {
            corners: [code.corners[0].into(), code.corners[1].into(), code.corners[2].into(), code.corners[3].into()],
            size: code.size as u32,
            cell_bitmap: code.cell_bitmap,
        }
    }
}

impl fmt::Debug for QuircCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("QuircCode")
            .field("corners", &&self.corners[..])
            .field("size", &self.size)
            .field("cell_bitmap", &&self.cell_bitmap[..])
            .finish()
    }
}

impl cmp::PartialEq for QuircCode {
    fn eq(&self, other: &QuircCode) -> bool {
        self.corners == other.corners &&                  // align
        self.size == other.size &&                        // align
        self.cell_bitmap[..] == other.cell_bitmap[..] &&  // align
        true
    }
}

impl cmp::Eq for QuircCode {}

impl cmp::Ord for QuircCode {
    fn cmp(&self, other: &QuircCode) -> cmp::Ordering {
        self.corners[..]
            .cmp(&other.corners)
            .then(self.size.cmp(&other.size))
            .then(self.cell_bitmap.cmp(&other.cell_bitmap))
    }
}

impl cmp::PartialOrd for QuircCode {
    fn partial_cmp(&self, other: &QuircCode) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl hash::Hash for QuircCode {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.corners.hash(state);
        self.size.hash(state);
        self.cell_bitmap.hash(state);
    }
}


/// This structure is used to return information about detected QR codes
/// in the input image.
#[derive(Copy, Clone)]
pub struct QuircData {
    /// Various parameters of the QR-code. These can mostly be
    /// ignored if you only care about the data.
    pub version: u32,
    pub ecc_level: u8,
    pub mask: u8,

    /// This field is the highest-valued data type found in the QR
    /// code.
    pub data_type: u8,

    /// Data payload. For the Kanji datatype, payload is encoded as
    /// Shift-JIS. For all other datatypes, payload is ASCII text.
    pub payload: [u8; QUIRC_MAX_PAYLOAD],
    pub payload_len: usize,

    /// ECI assignment number
    pub eci: u32,
}

impl From<FfiQuircData> for QuircData {
    fn from(data: FfiQuircData) -> QuircData {
        QuircData {
            version: data.version as u32,
            ecc_level: data.ecc_level as u8,
            mask: data.mask as u8,
            data_type: data.data_type as u8,
            payload: data.payload,
            payload_len: data.payload_len as usize,
            eci: data.eci,
        }
    }
}

impl fmt::Debug for QuircData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("QuircData")
            .field("version", &self.version)
            .field("ecc_level", &self.ecc_level)
            .field("mask", &self.mask)
            .field("data_type", &self.data_type)
            .field("payload", &&self.payload[..])
            .field("payload_len", &self.payload_len)
            .field("eci", &self.eci)
            .finish()
    }
}

impl cmp::PartialEq for QuircData {
    fn eq(&self, other: &QuircData) -> bool {
        self.version == other.version &&          // align
        self.ecc_level == other.ecc_level &&      // align
        self.mask == other.mask &&                // align
        self.data_type == other.data_type &&      // align
        self.payload[..] == other.payload[..] &&  // align
        self.payload_len == other.payload_len &&  // align
        self.eci == other.eci &&                  // align
        true
    }
}

impl cmp::Eq for QuircData {}

impl cmp::Ord for QuircData {
    fn cmp(&self, other: &QuircData) -> cmp::Ordering {
        self.version
            .cmp(&other.version)
            .then(self.ecc_level.cmp(&other.ecc_level))
            .then(self.mask.cmp(&other.mask))
            .then(self.data_type.cmp(&other.data_type))
            .then(self.payload[..].cmp(&other.payload))
            .then(self.payload_len.cmp(&other.payload_len))
            .then(self.eci.cmp(&other.eci))
    }
}

impl cmp::PartialOrd for QuircData {
    fn partial_cmp(&self, other: &QuircData) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl hash::Hash for QuircData {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.version.hash(state);
        self.ecc_level.hash(state);
        self.mask.hash(state);
        self.data_type.hash(state);
        self.payload.hash(state);
        self.payload_len.hash(state);
        self.eci.hash(state);
    }
}



#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct QuircRegion {
    pub seed: QuircPoint,
    pub count: u32,
    pub capstone: i32,
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct QuircCapstone {
    pub ring: u32,
    pub stone: u32,

    pub corners: [QuircPoint; 4],
    pub center: QuircPoint,
    pub c: [f64; QUIRC_PERSPECTIVE_PARAMS],

    pub qr_grid: i32,
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct QuircGrid {
    /// Capstone indices
    pub caps: [u32; 3],

    /// Alignment pattern region and corner
    pub align_region: i32,
    pub align: QuircPoint,

    /// Timing pattern endpoints
    pub tpep: [QuircPoint; 3],
    pub hscan: i32,
    pub vscan: i32,

    /// Grid size and perspective transform
    pub grid_size: u32,
    pub c: [f64; QUIRC_PERSPECTIVE_PARAMS],
}


#[derive(Clone)]
pub struct Quirc {
    pub(crate) image: Vec<u8>,
    pub(crate) pixels: Vec<u16>,
    /// used by threshold()
    pub(crate) row_average: Vec<u64>,
    pub(crate) w: usize,
    pub(crate) h: usize,

    pub(crate) num_regions: usize,
    pub(crate) regions: [QuircRegion; QUIRC_MAX_REGIONS],

    pub(crate) num_capstones: usize,
    pub(crate) capstones: [QuircCapstone; QUIRC_MAX_CAPSTONES],

    pub(crate) num_grids: usize,
    pub(crate) grids: [QuircGrid; QUIRC_MAX_GRIDS],
}

impl Quirc {
    /// Construct a new QR-code recognizer.
    pub fn new() -> Quirc {
        const NULL_POINT: QuircPoint = QuircPoint { x: 0, y: 0 };

        Quirc {
            image: Vec::new(),
            pixels: Vec::new(),
            row_average: Vec::new(),
            w: 0,
            h: 0,

            num_regions: 0,
            regions: [QuircRegion {
                seed: NULL_POINT,
                count: 0,
                capstone: 0,
            }; QUIRC_MAX_REGIONS],

            num_capstones: 0,
            capstones: [QuircCapstone {
                ring: 0,
                stone: 0,

                corners: [NULL_POINT; 4],
                center: NULL_POINT,
                c: [0f64; QUIRC_PERSPECTIVE_PARAMS],

                qr_grid: 0,
            }; QUIRC_MAX_CAPSTONES],

            num_grids: 0,
            grids: [QuircGrid {
                caps: [0; 3],

                align_region: 0,
                align: NULL_POINT,

                tpep: [NULL_POINT; 3],
                hscan: 0,
                vscan: 0,

                grid_size: 0,
                c: [0f64; QUIRC_PERSPECTIVE_PARAMS],
            }; QUIRC_MAX_GRIDS],
        }
    }

    /// Resize the QR-code recognizer. The size of an image must be
    /// specified before codes can be analyzed.
    pub fn resize(&mut self, to_w: usize, to_h: usize) {
        self.w = to_w;
        self.h = to_h;
        self.image.resize(to_w * to_h, 0u8);
        self.pixels = vec![0u16; to_w * to_h];
        self.row_average = vec![0u64; to_w];
    }

    /// Return the number of QR-codes identified in the last processed image.
    #[inline(always)]
    pub fn count(&self) -> usize {
        self.num_grids
    }

    /// Get the specified width and height of the QR code.
    #[inline(always)]
    pub fn size(&self) -> (usize, usize) {
        (self.w, self.h)
    }
}


// These traits are implemented manually, because rustc is at a loss for big arrays.
impl fmt::Debug for Quirc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Quirc")
            .field("image", &self.image)
            .field("pixels", &self.pixels)
            .field("row_average", &self.row_average)
            .field("w", &self.w)
            .field("h", &self.h)
            .field("num_regions", &self.num_regions)
            .field("regions", &&self.regions[..])
            .field("num_capstones", &self.num_capstones)
            .field("capstones", &self.capstones)
            .field("num_grids", &self.num_grids)
            .field("grids", &self.grids)
            .finish()
    }
}

impl cmp::PartialEq for Quirc {
    fn eq(&self, other: &Quirc) -> bool {
        self.image == other.image &&                  // align
        self.pixels == other.pixels &&                // align
        self.row_average == other.row_average &&      // align
        self.w == other.w &&                          // align
        self.h == other.h &&                          // align
        self.num_regions == other.num_regions &&      // align
        self.regions[..] == other.regions[..] &&      // align
        self.num_capstones == other.num_capstones &&  // align
        self.capstones == other.capstones &&          // align
        self.num_grids == other.num_grids &&          // align
        self.grids == other.grids &&                  // align
        true
    }
}

impl cmp::PartialOrd for Quirc {
    fn partial_cmp(&self, other: &Quirc) -> Option<cmp::Ordering> {
        self.image[..]
            .partial_cmp(&other.image)
            .and_then(|o| self.pixels.partial_cmp(&other.pixels).map(|n| o.then(n)))
            .and_then(|o| self.row_average.partial_cmp(&other.row_average).map(|n| o.then(n)))
            .and_then(|o| self.w.partial_cmp(&other.w).map(|n| o.then(n)))
            .and_then(|o| self.h.partial_cmp(&other.h).map(|n| o.then(n)))
            .and_then(|o| self.num_regions.partial_cmp(&other.num_regions).map(|n| o.then(n)))
            .and_then(|o| self.regions.partial_cmp(&other.regions).map(|n| o.then(n)))
            .and_then(|o| self.num_capstones.partial_cmp(&other.num_capstones).map(|n| o.then(n)))
            .and_then(|o| self.capstones.partial_cmp(&other.capstones).map(|n| o.then(n)))
            .and_then(|o| self.num_grids.partial_cmp(&other.num_grids).map(|n| o.then(n)))
            .and_then(|o| self.grids.partial_cmp(&other.grids).map(|n| o.then(n)))
    }
}
