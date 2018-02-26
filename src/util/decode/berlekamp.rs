use self::super::{MAX_POLY, GaloisField, poly_add};


/// Berlekamp-Massey algorithm for finding error locator polynomials.
pub fn berlekamp_massey(s: &[u8], big_n: usize, gf: &GaloisField) -> [u8; MAX_POLY] {
    let mut big_c = [0u8; MAX_POLY];
    let mut big_b = [0u8; MAX_POLY];

    let mut big_l = 0;
    let mut m = 1;

    let mut b = 1u8;

    big_b[0] = 1;
    big_c[0] = 1;

    for n in 0..big_n {
        let mut d = s[n];

        for i in 0..big_l + 1 {
            if !(big_c[i] != 0 && s[n - i] != 0) {
                continue;
            }

            d ^= gf.exp[((gf.log[big_c[i] as usize] + gf.log[s[n - i] as usize]) % gf.p as u8) as usize];
        }

        let mult = gf.exp[(gf.p - gf.log[b as usize] as usize + gf.log[d as usize] as usize) % gf.p];

        if d == 0 {
            m += 1;
        } else if big_l * 2 <= n {
            let big_t = big_c;

            poly_add(&mut big_c, &big_b, mult, m, gf);
            big_b = big_t;

            big_l = n + 1 - big_l;
            b = d;
            m = 1;
        } else {
            poly_add(&mut big_c, &big_b, mult, m, gf);
            m += 1;
        }
    }

    big_c
}
