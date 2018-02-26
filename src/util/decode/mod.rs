mod galois;
mod format;
mod decoder;
mod err_corr;
mod berlekamp;
mod polynominal;

pub use self::galois::{GaloisField, GF256_EXP, GF256_LOG, GF16_EXP, GF16_LOG, GF256, GF16};
pub use self::err_corr::{block_syndromes, correct_block, eloc_poly};
pub use self::polynominal::{poly_eval, poly_add};
pub use self::berlekamp::berlekamp_massey;
pub use self::decoder::{Datastream, codestream_ecc, decode_payload, decode_numeric, reserved_cell, numeric_tuple, decode_alpha, decode_kanji, decode_byte,
                        alpha_tuple, decode_eci, mask_bit};
pub use self::format::{FORMAT_MAX_ERROR, FORMAT_SYNDROMES, FORMAT_BITS, format_syndromes, correct_format};


pub const MAX_POLY: usize = 64;
