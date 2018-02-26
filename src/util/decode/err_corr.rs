//! Code stream error correction
//!
//! Generator polynomial for `GF(2^8)` is `x^8 + x^4 + x^3 + x^2 + 1`


use self::super::{GF256_EXP, GF256_LOG, MAX_POLY, GF256, berlekamp_massey, poly_eval};
use self::super::super::super::ops::version_db::QuircRsParams;
use self::super::super::super::ffi::QuircDecodeError;


pub fn block_syndromes(data: &[u8], bs: usize, npar: usize, s: &mut [u8]) -> bool {
    let mut nonzero = false;

    for i in 0..MAX_POLY {
        s[i] = 0;
    }

    for i in 0..npar {
        for j in 0..bs {
            let c = data[bs - j - 1];

            if c == 0 {
                continue;
            }

            s[i] ^= GF256_EXP[(GF256_LOG[c as usize] as usize + i * j) % 255];
        }

        if s[i] != 0 {
            nonzero = true;
        }
    }

    return nonzero;
}

pub fn eloc_poly(omega: &mut [u8], s: &[u8], sigma: &[u8], npar: usize) {
    for i in 0..MAX_POLY {
        omega[i] = 0;
    }

    for i in 0..npar {
        let a = sigma[i];

        if a == 0 {
            continue;
        }

        let log_a = GF256_LOG[a as usize] as usize;

        for j in 0..MAX_POLY - 1 {
            let b = s[j + 1];

            if i + j >= npar {
                break;
            }

            if b == 0 {
                continue;
            }

            omega[i + j] ^= GF256_EXP[(log_a + GF256_LOG[b as usize] as usize) % 255];
        }
    }
}

pub fn correct_block(data: &mut [u8], ecc: &QuircRsParams) -> QuircDecodeError {
    let npar = ecc.bs - ecc.dw;

    // Compute syndrome vector
    let mut s = [0u8; MAX_POLY];
    if !block_syndromes(&data, ecc.bs as usize, npar as usize, &mut s) {
        return QuircDecodeError::QuircSuccess;
    }

    let sigma = berlekamp_massey(&s, npar as usize, &GF256);

    // Compute derivative of sigma
    let mut sigma_deriv = [0u8; MAX_POLY];
    for i in 0..MAX_POLY - 1 {
        // Cool and good because step_by is hard apparently
        if i % 2 == 0 {
            sigma_deriv[i] = sigma[i + 1];
        }
    }

    // Compute error evaluator polynomial
    let mut omega = [0u8; MAX_POLY];
    eloc_poly(&mut omega, &s, &sigma, npar as usize - 1);

    // Find error locations and magnitudes
    for i in 0..ecc.bs {
        let xinv = GF256_EXP[255 - i as usize];

        if poly_eval(&sigma, xinv, &GF256) == 0 {
            let sd_x = poly_eval(&sigma_deriv, xinv, &GF256) as usize;
            let omega_x = poly_eval(&omega, xinv, &GF256) as usize;
            let error = GF256_EXP[((255 - GF256_LOG[sd_x] + GF256_LOG[omega_x]) % 255) as usize];

            data[(ecc.bs - i - 1) as usize] ^= error;
        }
    }

    if block_syndromes(data, ecc.bs as usize, npar as usize, &mut s) {
        QuircDecodeError::QuircErrorDataEcc
    } else {
        QuircDecodeError::QuircSuccess
    }
}
