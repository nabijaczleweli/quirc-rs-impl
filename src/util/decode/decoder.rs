//! Decoder algorithm


pub use self::super::super::super::constants::{QUIRC_DATA_TYPE_NUMERIC, QUIRC_DATA_TYPE_ALPHA, QUIRC_DATA_TYPE_KANJI, QUIRC_DATA_TYPE_BYTE, QUIRC_MAX_PAYLOAD};
use self::super::super::super::ops::version_db::{QUIRC_MAX_ALIGNMENT, QUIRC_MAX_VERSION, QUIRC_VERSION_DB};
pub use self::super::super::super::ops::{QuircCode, QuircData};
use self::super::super::super::ffi::QuircDecodeError;
use self::super::{correct_format, correct_block};


pub struct Datastream {
    pub raw: [u8; QUIRC_MAX_PAYLOAD],
    pub data_bits: u32,
    pub ptr: u32,

    pub data: [u8; QUIRC_MAX_PAYLOAD],
}

impl Datastream {
    fn bits_remaining(&self) -> usize {
        (self.data_bits - self.ptr) as usize
    }

    fn take_bits(&mut self, mut len: usize) -> u32 {
        let mut ret = 0;

        while len != 0 && self.ptr < self.data_bits {
            let b = self.data[(self.ptr >> 3) as usize];
            let bitpos = self.ptr & 7;

            ret <<= 1;
            if (b << bitpos) & 0x80 != 0 {
                ret |= 1;
            }

            self.ptr += 1;
            len -= 1;
        }

        ret
    }
}

impl QuircCode {
    #[inline]
    fn grid_bit(&self, x: u32, y: u32) -> u8 {
        let p = y * self.size + x;

        return (self.cell_bitmap[(p >> 3) as usize] >> (p & 7)) & 1;
    }

    fn read_format(&self, data: &mut QuircData, which: u32) -> QuircDecodeError {
        let mut format = 0u16;

        if which != 0 {
            for i in 0..7 {
                format = (format << 1) | self.grid_bit(8, self.size - 1 - i) as u16;
            }
            for i in 0..8 {
                format = (format << 1) | self.grid_bit(self.size - 8 + i, 8) as u16;
            }
        } else {
            static XS: [u32; 15] = [8, 8, 8, 8, 8, 8, 8, 8, 7, 5, 4, 3, 2, 1, 0];
            static YS: [u32; 15] = [0, 1, 2, 3, 4, 5, 7, 8, 8, 8, 8, 8, 8, 8, 8];

            for i in (0..15).rev() {
                format = (format << 1) | self.grid_bit(XS[i], YS[i]) as u16;
            }
        }

        format ^= 0x5412;

        let err = correct_format(&mut format);
        if err != QuircDecodeError::QuircSuccess {
            return err;
        }

        let fdata: u16 = format >> 10;
        data.ecc_level = (fdata >> 3) as u8;
        data.mask = (fdata & 7) as u8;

        QuircDecodeError::QuircSuccess
    }

    fn read_bit(&self, data: &mut QuircData, ds: &mut Datastream, i: u32, j: u32) {
        let bitpos = (ds.data_bits & 7) as usize;
        let bytepos = (ds.data_bits >> 3) as usize;
        let mut v = self.grid_bit(j, i);

        v ^= mask_bit(data.mask, i, j);

        if v != 0 {
            ds.raw[bytepos] |= 0x80 >> bitpos;
        }

        ds.data_bits += 1;
    }

    fn read_data(&self, data: &mut QuircData, ds: &mut Datastream) {
        let mut y = self.size as isize - 1;
        let mut x = self.size as isize - 1;
        let mut dir = -1;

        while x > 0 {
            if x == 6 {
                x -= 1;
            }

            if reserved_cell(data.version, y as u32, x as u32) == 0 {
                self.read_bit(data, ds, y as u32, x as u32);
            }

            if reserved_cell(data.version, y as u32, x as u32 - 1) == 0 {
                self.read_bit(data, ds, y as u32, x as u32 - 1);
            }

            y += dir;
            if y < 0 || y >= self.size as isize {
                dir = -dir;
                x -= 2;
                y += dir;
            }
        }
    }

    /// Decode a QR-code, returning the payload data
    pub fn decode(&self, data: &mut QuircData) -> QuircDecodeError {
        let mut ds = Datastream {
            raw: [0u8; QUIRC_MAX_PAYLOAD],
            data_bits: 0,
            ptr: 0,

            data: [0u8; QUIRC_MAX_PAYLOAD],
        };

        if (self.size - 17) % 4 != 0 {
            return QuircDecodeError::QuircErrorInvalidGridSize;
        }

        *data = QuircData {
            version: (self.size - 17) / 4,
            ecc_level: 0,
            mask: 0,

            data_type: 0,

            payload: [0u8; QUIRC_MAX_PAYLOAD],
            payload_len: 0,

            eci: 0,
        };

        if data.version < 1 || data.version as usize > QUIRC_MAX_VERSION {
            return QuircDecodeError::QuircErrorInvalidVersion;
        }

        // Read format information -- try both locations
        let mut err = self.read_format(data, 0);
        if err != QuircDecodeError::QuircSuccess {
            err = self.read_format(data, 1);
        }
        if err != QuircDecodeError::QuircSuccess {
            return err;
        }

        self.read_data(data, &mut ds);
        err = codestream_ecc(data, &mut ds);
        if err != QuircDecodeError::QuircSuccess {
            return err;
        }

        err = decode_payload(data, &mut ds);
        if err != QuircDecodeError::QuircSuccess {
            return err;
        }

        QuircDecodeError::QuircSuccess
    }
}

pub fn mask_bit(mask: u8, i: u32, j: u32) -> u8 {
    (match mask {
        0 => ((i + j) % 2) == 0,
        1 => (i % 2) == 0,
        2 => (j % 3) == 0,
        3 => ((i + j) % 3) == 0,
        4 => (((i / 2) + (j / 3)) % 2) == 0,
        5 => ((i * j) % 2 + (i * j) % 3) == 0,
        6 => (((i * j) % 2 + (i * j) % 3) % 2) == 0,
        7 => (((i * j) % 3 + (i + j) % 2) % 2) == 0,
        _ => false,
    }) as u8
}

pub fn reserved_cell(version: u32, i: u32, j: u32) -> u8 {
    let ver = &QUIRC_VERSION_DB[version as usize];
    let size = version * 4 + 17;

    // Finder + format: top left
    if i < 9 && j < 9 {
        return 1;
    }

    // Finder + format: bottom left
    if i + 8 >= size && j < 9 {
        return 1;
    }

    // Finder + format: top right
    if i < 9 && j + 8 >= size {
        return 1;
    }

    // Exclude timing patterns
    if i == 6 || j == 6 {
        return 1;
    }

    // Exclude version info, if it exists. Version info sits adjacent to
    // the top-right and bottom-left finders in three rows, bounded by
    // the timing pattern.
    if version >= 7 {
        if i < 6 && j + 11 >= size {
            return 1;
        }
        if i + 11 >= size && j < 6 {
            return 1;
        }
    }

    let mut ai = -1i32;
    let mut aj = -1i32;
    let mut a = 0i32;

    // Exclude alignment patterns
    while a < QUIRC_MAX_ALIGNMENT as i32 && ver.apat[a as usize] != 0 {
        let p = ver.apat[a as usize];

        if (p as i32 - i as i32).abs() < 3 {
            ai = a;
        }
        if (p as i32 - j as i32).abs() < 3 {
            aj = a;
        }

        a += 1;
    }

    if ai >= 0 && aj >= 0 {
        a -= 1;
        if ai > 0 && ai < a {
            return 1;
        }
        if aj > 0 && aj < a {
            return 1;
        }
        if aj == a && ai == a {
            return 1;
        }
    }

    0
}

pub fn codestream_ecc(data: &mut QuircData, ds: &mut Datastream) -> QuircDecodeError {
    let ver = &QUIRC_VERSION_DB[data.version as usize];
    let sb_ecc = &ver.ecc[data.ecc_level as usize];
    let lb_count = (ver.data_bytes - sb_ecc.bs * sb_ecc.ns) / (sb_ecc.bs + 1);
    let bc = lb_count + sb_ecc.ns;
    let ecc_offset = sb_ecc.dw * bc + lb_count;
    let mut dst_offset = 0;

    let mut lb_ecc = *sb_ecc;
    lb_ecc.dw += 1;
    lb_ecc.bs += 1;

    for i in 0..bc {
        let dst = &mut ds.data[dst_offset..];
        let ecc = if i < sb_ecc.ns { sb_ecc } else { &lb_ecc };
        let num_ec = ecc.bs - ecc.dw;

        for j in 0..ecc.dw {
            dst[j as usize] = ds.raw[(j * bc + i) as usize];
        }
        for j in 0..num_ec {
            dst[(ecc.dw + j) as usize] = ds.raw[(ecc_offset + j * bc + i) as usize];
        }

        let err = correct_block(dst, ecc);
        if err != QuircDecodeError::QuircSuccess {
            return err;
        }

        dst_offset += ecc.dw as usize;
    }

    ds.data_bits = dst_offset as u32 * 8;

    QuircDecodeError::QuircSuccess
}

pub fn numeric_tuple(data: &mut QuircData, ds: &mut Datastream, bits: usize, digits: usize) -> bool {
    if ds.bits_remaining() < bits {
        return true;
    }

    let mut tuple = ds.take_bits(bits);

    for i in (0..digits).rev() {
        data.payload[data.payload_len + i] = (tuple % 10 + b'0' as u32) as u8;
        tuple /= 10;
    }

    data.payload_len += digits;

    false
}

pub fn decode_numeric(data: &mut QuircData, ds: &mut Datastream) -> QuircDecodeError {
    let bits = if data.version < 10 {
        10
    } else if data.version < 27 {
        12
    } else {
        14
    };

    let mut count = ds.take_bits(bits);
    if data.payload_len + count as usize + 1 > QUIRC_MAX_PAYLOAD {
        return QuircDecodeError::QuircErrorDataOverflow;
    }

    while count >= 3 {
        if numeric_tuple(data, ds, 10, 3) {
            return QuircDecodeError::QuircErrorDataUnderflow;
        }
        count -= 3;
    }

    if count >= 2 {
        if numeric_tuple(data, ds, 7, 2) {
            return QuircDecodeError::QuircErrorDataUnderflow;
        }
        count -= 2;
    }

    if count != 0 {
        if numeric_tuple(data, ds, 4, 1) {
            return QuircDecodeError::QuircErrorDataUnderflow;
        }
    }

    QuircDecodeError::QuircSuccess
}

pub fn alpha_tuple(data: &mut QuircData, ds: &mut Datastream, bits: usize, digits: usize) -> bool {
    if ds.bits_remaining() < bits {
        return true;
    }

    let mut tuple = ds.take_bits(bits);

    for i in 0..digits {
        static ALPHA_MAP: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ $%*+-./:";

        data.payload[data.payload_len + digits - i - 1] = ALPHA_MAP[(tuple % 45) as usize];
        tuple /= 45;
    }

    data.payload_len += digits;

    false
}

pub fn decode_alpha(data: &mut QuircData, ds: &mut Datastream) -> QuircDecodeError {
    let bits = if data.version < 10 {
        9
    } else if data.version < 27 {
        11
    } else {
        13
    };

    let mut count = ds.take_bits(bits);
    if data.payload_len + count as usize + 1 > QUIRC_MAX_PAYLOAD {
        return QuircDecodeError::QuircErrorDataOverflow;
    }

    while count >= 2 {
        if alpha_tuple(data, ds, 11, 2) {
            return QuircDecodeError::QuircErrorDataUnderflow;
        }
        count -= 2;
    }

    if count != 0 {
        if alpha_tuple(data, ds, 6, 1) {
            return QuircDecodeError::QuircErrorDataUnderflow;
        }
    }

    QuircDecodeError::QuircSuccess
}

pub fn decode_byte(data: &mut QuircData, ds: &mut Datastream) -> QuircDecodeError {
    let bits = if data.version < 10 { 8 } else { 16 };

    let count = ds.take_bits(bits);
    if data.payload_len + count as usize + 1 > QUIRC_MAX_PAYLOAD {
        return QuircDecodeError::QuircErrorDataOverflow;
    }
    if ds.bits_remaining() < count as usize * 8 {
        return QuircDecodeError::QuircErrorDataUnderflow;
    }

    for _ in 0..count {
        data.payload[data.payload_len] = ds.take_bits(8) as u8;
        data.payload_len += 1;
    }

    QuircDecodeError::QuircSuccess
}

pub fn decode_kanji(data: &mut QuircData, ds: &mut Datastream) -> QuircDecodeError {
    let bits = if data.version < 10 {
        8
    } else if data.version < 27 {
        10
    } else {
        12
    };

    let count = ds.take_bits(bits);
    if data.payload_len + count as usize * 2 + 1 > QUIRC_MAX_PAYLOAD {
        return QuircDecodeError::QuircErrorDataOverflow;
    }
    if ds.bits_remaining() < count as usize * 13 {
        return QuircDecodeError::QuircErrorDataUnderflow;
    }

    for _ in 0..count {
        let d = ds.take_bits(13);
        let ms_big_b = d / 0xc0;
        let ls_big_b = d % 0xc0;
        let intermediate = (ms_big_b << 8) | ls_big_b;

        let sjw = if intermediate + 0x8140 <= 0x9ffc {
            // bytes are in the range 0x8140 to 0x9FFC
            intermediate + 0x8140
        } else {
            // bytes are in the range 0xE040 to 0xEBBF
            intermediate + 0xc140
        };

        data.payload[data.payload_len] = ((sjw >> 8) & 0xff) as u8;
        data.payload[data.payload_len + 1] = (sjw & 0xff) as u8;
        data.payload_len += 2;
    }

    QuircDecodeError::QuircSuccess
}

pub fn decode_eci(data: &mut QuircData, ds: &mut Datastream) -> QuircDecodeError {
    if ds.bits_remaining() < 8 {
        return QuircDecodeError::QuircErrorDataUnderflow;
    }

    data.eci = ds.take_bits(8);

    if (data.eci & 0xc0) == 0x80 {
        if ds.bits_remaining() < 8 {
            return QuircDecodeError::QuircErrorDataUnderflow;
        }

        data.eci = (data.eci << 8) | ds.take_bits(8);
    } else if (data.eci & 0xe0) == 0xc0 {
        if ds.bits_remaining() < 16 {
            return QuircDecodeError::QuircErrorDataUnderflow;
        }

        data.eci = (data.eci << 16) | ds.take_bits(16);
    }

    QuircDecodeError::QuircSuccess
}

pub fn decode_payload(data: &mut QuircData, ds: &mut Datastream) -> QuircDecodeError {
    fn done(data: &mut QuircData) {
        // Add nul terminator to all payloads
        if data.payload_len >= data.payload.len() {
            data.payload_len -= 1;
        }
        data.payload[data.payload_len] = 0;
    }


    while ds.bits_remaining() >= 4 {
        let tpe = ds.take_bits(4) as u8;

        let err = match tpe {
            QUIRC_DATA_TYPE_NUMERIC => decode_numeric(data, ds),

            QUIRC_DATA_TYPE_ALPHA => decode_alpha(data, ds),

            QUIRC_DATA_TYPE_BYTE => decode_byte(data, ds),

            QUIRC_DATA_TYPE_KANJI => decode_kanji(data, ds),

            7 => decode_eci(data, ds),

            _ => {
                done(data);
                return QuircDecodeError::QuircSuccess;
            }
        };

        if err != QuircDecodeError::QuircSuccess {
            return err;
        }

        if (tpe & (tpe - 1)) == 0 && tpe > data.data_type {
            data.data_type = tpe;
        }
    }

    done(data);
    QuircDecodeError::QuircSuccess
}
