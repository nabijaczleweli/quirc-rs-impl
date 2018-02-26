pub mod version_db;

use self::super::ffi::FfiQuircPoint;
use std::{cmp, fmt, u16};


const QUIRC_MAX_REGIONS: usize = 256;
const QUIRC_MAX_CAPSTONES: usize = 32;
const QUIRC_MAX_GRIDS: usize = 8;
const QUIRC_PERSPECTIVE_PARAMS: usize = 8;


/// This structure describes a location in the input image buffer.
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct QuircPoint {
    x: usize,
    y: usize,
}

impl From<FfiQuircPoint> for QuircPoint {
    fn from(code: FfiQuircPoint) -> QuircPoint {
        QuircPoint {
            x: code.x as usize,
            y: code.y as usize,
        }
    }
}


#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct QuircRegion {
    seed: QuircPoint,
    count: u32,
    capstone: u32,
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct QuircCapstone {
    ring: u32,
    stone: u32,

    corners: [QuircPoint; 4],
    center: QuircPoint,
    c: [f64; QUIRC_PERSPECTIVE_PARAMS],

    qr_grid: u32,
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct QuircGrid {
    /* Capstone indices */
    caps: [u32; 3],

    /* Alignment pattern region and corner */
    align_region: u32,
    align: QuircPoint,

    /* Timing pattern endpoints */
    tpep: [QuircPoint; 3],
    hscan: u32,
    vscan: u32,

    /* Grid size and perspective transform */
    grid_size: u32,
    c: [f64; QUIRC_PERSPECTIVE_PARAMS],
}


#[derive(Clone)]
pub struct Quirc {
    image: Vec<u8>,
    pixels: Vec<u16>,
    row_average: Vec<u64>, // used by threshold()
    w: usize,
    h: usize,

    num_regions: usize,
    regions: [QuircRegion; QUIRC_MAX_REGIONS],

    num_capstones: usize,
    capstones: [QuircCapstone; QUIRC_MAX_CAPSTONES],

    num_grids: usize,
    grids: [QuircGrid; QUIRC_MAX_GRIDS],
}

impl Quirc {
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

    pub fn resize(&mut self, to_w: usize, to_h: usize) {
        self.w = to_w;
        self.h = to_h;
        self.image.resize(to_w * to_h, 0u8);
        self.pixels = vec![0u16; to_w * to_h];
        self.row_average = vec![0u64; to_w];
    }

    #[inline(always)]
    pub fn count(&self) -> usize {
        self.num_grids
    }

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
