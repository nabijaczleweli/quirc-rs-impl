//! Format value error correction
//!
//! Generator polynomial for `GF(2^4)` is `x^4 + x + 1`


use self::super::{GF16_EXP, MAX_POLY, GF16, berlekamp_massey, poly_eval};
use self::super::super::super::ffi::QuircDecodeError;


pub const FORMAT_MAX_ERROR: usize = 3;
pub const FORMAT_SYNDROMES: usize = FORMAT_MAX_ERROR * 2;
pub const FORMAT_BITS: usize = 15;


pub fn format_syndromes(u: u16, s: &mut [u8]) -> bool {
    let mut nonzero = false;

    for i in 0..MAX_POLY {
        s[i] = 0;
    }

    for i in 0..FORMAT_SYNDROMES {
        s[i] = 0;

        for j in 0..FORMAT_BITS {
            if u & (1 << j) != 0 {
                s[i] ^= GF16_EXP[((i + 1) * j) % 15];
            }
        }

        if s[i] != 0 {
            nonzero = true;
        }
    }

    nonzero
}

pub fn correct_format(f_ret: &mut u16) -> QuircDecodeError {
    let mut u = *f_ret;
    let mut s = [0u8; MAX_POLY];

    // Evaluate U (received codeword) at each of alpha_1 .. alpha_6
    // to get S_1 .. S_6 (but we index them from 0).
    if !format_syndromes(u, &mut s) {
        return QuircDecodeError::QuircSuccess;
    }

    let sigma = berlekamp_massey(&s, FORMAT_SYNDROMES, &GF16);

    // Now, find the roots of the polynomial
    for i in 0..15 {
        if poly_eval(&sigma, GF16_EXP[15 - i], &GF16) == 0 {
            u ^= 1 << i;
        }
    }

    if format_syndromes(u, &mut s) {
        return QuircDecodeError::QuircErrorFormatEcc;
    }

    *f_ret = u;
    QuircDecodeError::QuircSuccess
}
