use self::super::super::super::ops::Quirc;


const FLOOD_FILL_MAX_DEPTH: usize = 4096;


impl Quirc {
    /// Span-based floodfill routine
    //                              y      left   right
    pub(crate) fn flood_fill_seed<F: FnMut(usize, usize, usize)>(&mut self, x: usize, y: usize, from: u16, to: u16, mut func: F, depth: usize) {
        self.actually_flood_fill_seed(x, y, from, to, &mut func, depth)
    }

    fn actually_flood_fill_seed<F: FnMut(usize, usize, usize)>(&mut self, x: usize, y: usize, from: u16, to: u16, func: &mut F, depth: usize) {
        if depth >= FLOOD_FILL_MAX_DEPTH {
            return;
        }

        let mut left = x;
        let mut right = x;

        {
            let row = &mut self.pixels[(y * self.w)..];

            while left > 0 && row[left - 1] == from {
                left -= 1;
            }

            while right < self.w - 1 && row[right + 1] == from {
                right += 1;
            }

            /* Fill the extent */
            for i in left..(right + 1) {
                row[i] = to;
            }
        }

        func(y, left, right);

        // Seed new flood-fills
        if y > 0 {
            for i in left..(right + 1) {
                if self.pixels[((y - 1) * self.w)..][i] == from {
                    self.actually_flood_fill_seed(i, y - 1, from, to, func, depth + 1);
                }
            }
        }

        if y < self.h - 1 {
            for i in left..(right + 1) {
                if self.pixels[((y + 1) * self.w)..][i] == from {
                    self.actually_flood_fill_seed(i, y + 1, from, to, func, depth + 1);
                }
            }
        }
    }
}
