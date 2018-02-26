//! Polynomial operations


use self::super::{MAX_POLY, GaloisField};


pub fn poly_add(dst: &mut [u8], src: &[u8], c: u8, shift: isize, gf: &GaloisField) {
    if c == 0 {
        return;
    }

    let log_c = gf.log[c as usize];

    for i in 0..MAX_POLY {
        let p = i as isize + shift;
        let v = src[i];

        if p < 0 || p >= MAX_POLY as isize {
            continue;
        }
        if v == 0 {
            continue;
        }

        dst[p as usize] ^= gf.exp[(gf.log[v as usize] + log_c) as usize % gf.p];
    }
}

pub fn poly_eval(s: &[u8], x: u8, gf: &GaloisField) -> u8 {
    if x == 0 {
        return s[0];
    }

    let mut sum = 0u8;
    let log_x = gf.log[x as usize] as usize;

    for i in 0..MAX_POLY {
        let c = s[i];

        if c == 0 {
            continue;
        }

        sum ^= gf.exp[(gf.log[c as usize] as usize + log_x * i) % gf.p];
    }

    sum
}
